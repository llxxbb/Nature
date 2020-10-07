# Built-in executor

In normal case, `Executor` needs to be implemented by developers. For some commonly used general functions, Nature make it as built-in to reduce the burden on developers. For `Executor` please refer to [Executor](executor.md) and [Relation](relation.md).

As Nature's application field continues to discover, the built-in `Executor` will continue to be enriched and improved, so this document may be adjusted frequently.

Three types of `Executor` are defined in [Executor](executor.md): `converter`, `convert_before` and `convert_after`. The following content is organized according to these three interfaces.

## converter interface

### scatter

**Function**: Split the upstream Instance into multiple downstream Instances. And use `Instance.para` to distinguish these downstream `Instance`s.

**Upstream Instance requirements**: The `Instance.content` must be in the following json format, **Note**: If the data format does not meet the requirements, you can use `convert_before` to correct it.

```json
[
  {
    "key": "a/1",
    "value": 33
  },
  {
    "key": "a/2",
    "value": 76
  },
  ...
]
```

**Downstream Instance Description**: The value corresponding to the `key` in the upstream will become the `Instance.para` of the downstream Instance, and the value corresponding to the `value` will be put into the `Instance.content` of the downstream. For example, the json above will generate the following `Instance`:

| meta           | para | content |
| -------------- | ---- | ------- |
| B:downstream:1 | a/1  | 33      |
| B:downstream:1 | a/2  | 76      |

**Options**:

| Option              | Description                                                  | Default value |
| ------------------- | ------------------------------------------------------------ | ------------- |
| dimension_separator | If the key value of the `Instance.content` is not the separated of the `Instance.para`,   you need to set this property. For example, the key is "a-1", this attribute should be set to "-". | /             |

### merge

**Function**: merge multiple upstream data, or merge one upstream data into multiple data in downstream. **Note**: Merge can only handle **integer** data.

**Options**:

| Option    | Description                                                  | Example   | Default Value |
| --------- | ------------------------------------------------------------ | --------- | ------------- |
| key       | There are three modes: Para, None and Content. See below for details | See below | None          |
| when_same | how to handle the same `key`:<br />Old: Keep the old value<br />New: Keep the new value<br />Min: Take the smallest one<br />Max: Take the largest one <br />Sum: Take the sum of the two | "Old"     | "Sum"         |
| sum_all   | whether to add the total field to the result                 | true      | false         |
| top       | MaxTop(u16) reverse order topN<br />MinTop(u16) positive order topN<br />None No need for top, | MaxTop(5) | None          |

#### Para mode

To merge the upstream `Instance` into the downstream `Instance`, there are the following requirements:

- The `Meta` of the downstream `Instance` must be stateful so can store the merged result. Because the downstream is stateful, this mode is not suitable for use in situations with a large number of data to merge; on the one hand, there is a problem with the capacity of the downstream itself, on the other hand, it is a performance problem for the large number of merged versions.
- The `key` of the merge item comes from upstream `Instance.para`
- The `value` of the merge item comes from upstream `Instance.content`

The example settings are as follows:

```json
{"key":{"Para":[0]},"sum_all":true}
```

The upstream example is as follows. According to the settings, `key` is b and `value` is 76.

| meta-string  | para | content |
| ------------ | ---- | ------- |
| B:upstream:1 | b    | 76      |

The downstream example data is as follows:

| meta-string  | content                         | state_version |
| ------------ | ------------------------------- | ------------- |
| B:sum:1\|0\| | {"detail":{"a":37},"total":100} | 1             |

The merged downstream data is as follows:

| meta-string | content                                  | state_version |
| ----------- | ---------------------------------------- | ------------- |
| B:sum:1\    | {"detail":{"a":37, "b":76}, "total":112} | 2             |

#### None mode

The statistical item has no key. This mode merge the upstream numeric arrays and outputs the result as the downstream `Instance`.

Examples of upstream is as follows.

| meta-string  | id   | content       |
| ------------ | ---- | ------------- |
| B:upstream:1 | 123  | ["1","3","5"] |

The combined downstream data is as follows:

| meta-string | id   | content |
| ----------- | ---- | ------- |
| B:sum:1     | 123  | 9       |

#### Content Mode

merge multiple upstream data, each data is [key, value] structure

Examples of option settings are as follows:

```json
{"key":"Content","when_same":"Old"}
```

The upstream example `Instance` is as follows. Note that `content` is an array of strings, and each string is an array with only two elements, the first is the `key` and the second is the `value`.

| meta-string  | id   | content                                       |
| ------------ | ---- | --------------------------------------------- |
| B:upstream:1 | 123  | ["[\\"a\\":10]","[\\"b\\":2]","[\\"a\\":8] "] |

The merged downstream is as follows:

| meta-string | id   | content         |
| ----------- | ---- | --------------- |
| B:old:1     | 123  | {"a":10, "b":2} |

If the value of `when_same` is changed from Old to Min or New, the value of `content` will be: {"a":8, "b":2}

## convert_before interface

### para_as_key

**Function**: Use a part of `Instance.para` as the key, and use `nstant.content` as the value to form [key, value] and replace the original `Instance.content`.

**Options**:

| Option | Description                                                  | Example | Default Value |
| ------ | ------------------------------------------------------------ | ------- | ------------- |
| plain  | If false, the value of `value` will be put in double quotes. | true    | false         |
| part   | Which parts of the upstream `Instance.para` are used as input | [1]     |               |

**Example**:

Instance.para = "ll/xx/bb"

Instance.content = 123

If configured as

```json
{
	"plain":true,
	"part":[1]
}
```

After executed, Instance.content becomes: ["xx",123]

If configured as

```json
{
"part":[0,2]
}
```

After executed, Instance.content becomes: ["ll/bb","123"], note that 123 is double quoted

### time_range

**Function**: Used to generate a time range (unit: seconds) and assign it to the `Instance.para` property.

**Option**:

| Option    | Description                                                  | Example   | Default Value |
| --------- | ------------------------------------------------------------ | --------- | ------------- |
| unit      | Interval span: s(econd), m(inute), h(our), d(ay), w(eek), M(onth), Y(ear) | see below | "s"           |
| value     | If the `unit` value is "s","m","h","d", then `value` is the number of intervals (must be> 0)<br />If the unit value is "w","M ","y" then `value` is the offset; value can be <0, which means offset from the tail to the front. The range of values are as follows:<br />week: [-7, 6]<br /> month: [-20, 19]<br /> year: [-200, 199] | 5         | 1             |
| on_para   | Where to take the time for calculation, if it is true, take it from the upstream `Instance.para`, otherwise take `Instance.create_time` | true      | false         |
| time_part | Which part of the upstream `Instance.para` to take as the input time | 1         | 0             |

Example of generating para: 1596207504/1596207505

### instance-loader

**Function**: Retrieve a batch of `Instance` according to the given conditions, extract all the `content` attributes to form an array, and replace the `from.content`of parameter [ConverterParameter](data-define.md) .

**Options**:

The ins_key structure used below is: meta-string:id:para.

| Option    | Description                                                  | Example                                                      | Default Value |
| --------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------- |
| key_gt    | To form SQL where conditions: ins_key> {key_gt}              | B:sale/item/count:1\|0\|(item)/                              |               |
| key_lt    | To form SQL where conditions: ins_key <{key_lt}              | B:sale/item/count:1\|0\|(item)0                              |               |
| page_size | The size of a page of data                                   | 20                                                           | 100           |
| time_part | Which two parts of upstream `Instance.para` are used as the start and end time | [0,1]                                                        |               |
| filters   | is an array of `Executor`, each is the definition of the convert_before interface, used for processing after data loaded. | [{"protocol":"localRust","url":"nature_integrate_test_executor:append_star"}, {"protocol":"localRust","url":"nature_integrate_test_executor:append_plus"}] |               |

### task-checker

**Function**: Before executing `Executor`, check whether the related Nature task is completed. It is generally used to determine whether the required data is fully in place when loading instances. If it is not in place, return environment exception and wait for the next retry.

**Options**:

| Option    | Description                                                  | Example                         | Default Value |
| --------- | ------------------------------------------------------------ | ------------------------------- | ------------- |
| key_gt    | To form SQL where conditions task_key> {key_gt}              | B:sale/item/count:1\|0\|(item)/ |               |
| key_lt    | To form SQL where conditions task_key< {key_lt}              | B:sale/item/count:1\|0\|(item)0 |               |
| time_part | Which two parts of upstream `Instance.para` are used as the start and end time | [0,1]                           |               |