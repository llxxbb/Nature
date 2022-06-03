# Relation

It is used to define the upstream and downstream relationship between two `Meta`, and its definition stored in the data table: relation.

## 定义 `Relation`

As an example:

```sql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:sale/orderState:1', '{"target_states":{"add":["new"]}}');
```

## Define how to handle `Relation`

Even if two `Meta` have established `Relation`, it may not be executed. It depends on the `settings`. `settings` is a JSON string and its content defined as follows:

```json
{
    "selector": {...},             // default null, select the upstream that meets the conditions. See "Selector" below
    "executor": {...}, // default null, specify Executor. See "Executor" below
    "convert_before": [{...}],     // Pre-executor, you can specify multiple, will be execute in the given order.
    "convert_after": [{...}],     // Post Executor, you can specify multiple, will be execute in the given order.
    "use_upstream_id": bool,     // The newly generated Instance.id will use the upstream Instance.id
    "target": {},                 // default null, for downstream Instance intervention, see "downstream Intervention" below
    "delay": 0,                 // default 0, the task will be executed after the specified number of seconds from the current time
    "delay_on_para": [100,2],     // default null, delay execution. The first value of the array is the delay in seconds, and the second value is the position of the base time, which is located in the upstream Instance.para.
    "id_bridge": bool,             // default false, the upstream id is not used downstream, but the downstream of the downstream will use it, then you need to set this value to true
}
```

### Selector

The upstream and downstream must meet the specified conditions before Nature can call Executor. These conditions defined as follows:

```json
{
     "state_all": ["s1"], // default null, upstream must meet all specified states
     "state_any": ["s1"], // default null, upstream needs to satisfy one of the states
     "state_none": ["s1"], // default null, upstream cannot contain any given state
     "last_all": ["s1"], // default null, the downstream previous version must meet all specified states
     "last_any": ["s1"], // default null, the downstream previous version needs to meet one of the states
     "last_none": ["s1"], // default null, the downstream previous version cannot contain any given status
    "context_all": ["c1"], // default null, upstream must meet all specified context
     "context_any": ["c1"], // default null, upstream needs to satisfy one of the context
     "context_none": ["c1"], // default null, upstream cannot contain any given context
     "sys_context_all": ["c1"], // default null, upstream must meet all specified sys_context
     "sys_context_any": ["c1"], // default null, upstream needs to meet one of the sys_context
     "sys_context_none": ["c1"], // default null, upstream cannot contain any given sys_context
}
```

The check order of conditions is: xxx_none, xxx_all, xxx_any.

**Note**: If last_xxx is not satisfied, an `EnvError` will be generated and try it again later.

**Note**: Although both `context` and `sys_context` are KV types, when used as process selection conditions, Nature only handles the "K" but not the "V". This is considered for easy design. The form of "V" is determined by the business, it may be a URL,  "a|b|c" or a json, so it is not standardized. Nature also does not want to regulate this, which may limit business flexibility and reduce processing performance, but the "K" is very standardized, just a label, which is very convenient for Nature to process. Of course, there are problems with this approach. When `context` and `sys_context` are used as process choices, they lose the meaning of KV. For example: choosing different processing procedures according to gender:

- Wrong way：
  
  | KEY    | VALUE           |
  | ------ | --------------- |
  | gender | "boy" \| "girl" |

- Correct way 1：
  
  | KEY                       | VALUE |
  | ------------------------- | ----- |
  | gender.boy \| gender.girl | ""    |
  
  The flow control settings are similar to:
  
  - boy flow：relation1.selector.**context_all** = ["gender.boy"]
  
  - girl flow：relation2.selector.**context_all** = ["gender.girl"]

- Correct way 2：
  
  | KEY          | VALUE |
  | ------------ | ----- |
  | gender.isBoy | ""    |
  
  The flow control settings are similar to:
  
  - boy flow：relation1.selector.**context_all** = ["gender.isBoy"]
  
  - girl flow：relation2.selector.**context_none** = ["gender.isBoy"]

### Executor

`Executor` has three forms currently: converter, pre-filter, and post-filter. Its configuration adopts the following form.

```json
{
     "protocol": "http",             // Communication protocol, see description below.
     "url": "http://my-executor/fun", // used to locate the of Executor
     "settings": "executor self settings", // see the description below.
}
```

**protocol**: The communication protocol between Nature and ʻExecutor`. Its value is not case-sensitive. The following methods supported currently.

- Http | Https: Call an `Executor` remotely via post.
- LocalRust: `Executor` is implemented as a Rust library, and Nature interacts with this library via FFI.
- Auto: When you do not specify an `executor`, Nature will automatically construct an `executor` at `runtime`, but the `auto-executor` has no ability to generate content for `Instance.content`. So when we only care about ID, status and other information, Nature's `auto-executor` will bring us a lot of convenience.
- BuiltIn: Use the built-in converter supplied by Nature. Specify a `builtin-executor` will be used through the `url` attribute.

Both http and LocalRust required to be implemented by yourselves, please refer to [Executor interface](executor.md)。

**settings**: Each `Executor` can have its own independent configuration, which is explained by itself. **Note** The content of settings can be replaced by the content in the `para.dynamic` property of `Instance.sys_context` at `runtime`, and this replacement limited to the current Instance and will not affect other Instances. Example: Suppose the settings of a before_filter used to load batch Instances are as follows:

```json
{
    "key_gt":"B:sale/item/(item_id):1|",
    "key_lt":"B:sale/item/(item_id):2|"
}
```

We want (item_id) to be replaced by the real ID at `runtime`. At this time, if the `para.dynamic` attribute of upstream `instance.sys_context` contains the following definition, our wish can be realized:

```properties
para.dynamic = "[[\"(item_id)\":\"123\"]]"
```

Before Nature calls `Executor`, it will replace `settings` with the following content and pass it to `Exexutor`

```json
{
    "key_gt":"B:sale/item/123:1|",
    "key_lt":"B:sale/item/123:2|"
}
```

**Note**: Currently `para.dynamic` only supports simple substitutions. It is recommended to add clear boundary characters, such as "()" in this example to avoid incorrect substitutions.

**Some examples of `Executor`**

```json
{
    "protocol":"Http",
    "url":"http://some_domain:8081/some_converter"
}
```

```json
{
    "protocol":"LocalRust",
    "url":"some_lib:some_converter"
}
```

```json
{
    "protocol":"builtIn",
    "url":"sum"
}
```

### Intervene downstream

After the execution of `Executor` is completed, sometimes we want to append some information to the target `Instance`. For example, for the initial `Order`, we can automatically set the state of `OrderState` to `new` without programming. At this time, we can adjust the result through the following configuration.

```json
{
     "state_add": ["s1"],         // Default null, add a new state based on the previous state, the state must be defined in `Meta`.
     "state_remove": ["s1"],     // Default null, remove the specified state from the previous state.
     "append_para": [2,1],         // Default null, select a part from the upstream Instance.para and append it to the downstream Instance.para. See below for details.
     "dynamic_para": String,    // see description below
}
```

**append_para**: This attribute explains how to generate downstream `Instance.para` attributes. For example, if its value is [3,1], assuming upstream `Instance.para` is “a/b/c/d”, then the value of `para` of the downstream instance is “d/b”. If the downstream `para` already has a value, append it after that value. **Note** If the downstream `Meta` is status data, its own **para** cannot have a value, otherwise the version data cannot be formed.

**dynamic_para**: Nature will use this to generate the `para.dynamic` of the `Instance.sys_context` attribute, the format is as follows:

```json
{"para.dynamic":"[[\"key\",\"value\"]]"}
```

The key derived from the value corresponding to `dynamic_para`, and the value derived from the additional value generated by `append_para`. The function of `para.dynamic` is to replace the variables in `Executor.settings`, please refer to the sales statistics in [Demo](../../../nature-demo/README_EN.md).