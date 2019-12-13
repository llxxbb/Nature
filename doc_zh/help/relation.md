# Relation

## 存储 `Relation`

关系数据被存储到 relation 数据表中。示例如下：

```sql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:sale/orderState:1', '{"target_states":{"add":["new"]}}');
```

## 定义 `Relation` 的处理方式

上面的示例 SQL 中的 settings 字段用于对每个关系的处理方式进行个性化定义。settings 的值是 JSON形式的 `RelationSettings` 对象，其结构如下。

```rust
pub struct RelationSettings {
    pub selector: Option<FlowSelector>,
    pub executor: Option<Vec<Executor>>,
    pub use_upstream_id: bool,
    pub target_states: Option<TargetState>,
    pub delay: i32,
}
```

- selector：属性用于选择符合条件的 `Instance` 进入 `Converter` 进入处理，其结构见下方 `FlowSelector`的结构说明。
- executor：属性用于定义谁来做这个转化处理，其结构见下方 `Executor`的结构说明。
- use_upstream_id：新生成的 `Instance` 的 ID 将使用上游 `Instance`的 ID。
- target_states：可以增加或删除转化后的 `Instance` 的状态，状态必须在 `Meta` 中定义过。
- delay：本次转化需要延迟指定的秒数后执行。

### 触发转换的条件： FlowSelector

```rust
pub struct FlowSelector {
    pub source_state_include: HashSet<String>,
    pub source_state_exclude: HashSet<String>,
    pub target_state_include: HashSet<String>,
    pub target_state_exclude: HashSet<String>,
    pub context_include: HashSet<String>,
    pub context_exclude: HashSet<String>,
}
```

- source_state_include：上游 `Instance` 的状态中必须包含指定的状态。
- source_state_exclude：上游 `Instance` 的状态中不能包含指定的状态。
- target_state_include：上一版本生成的 `Instance`的状态中必须包含指定的状态。
- target_state_exclude：上一版本生成的 `Instance`的状态中不能包含指定的状态。
- context_include：上游`Instance` 上下文中必须包含指定的上下文。
- context_exclude：上游`Instance` 上下文中不能包含指定的上下文。

## 定义用于转化的：Executor

```rust
pub struct Executor {
    pub protocol: Protocol,
    pub url: String,
    pub group: String,
    pub proportion: u32,
}
```

protocol： Nature 与 执行器间的通讯协议，目前支持下面的方式。

- Http | Https：远程调用一个`Executor`。

- LocalRust：Nature 会加载一个本地 rust 库作为`Executor`

- Auto：不需要显式设置此值。如果 `RelationSettings` 中没有定义`executor`属性则 Nature 会自动进行转换操作。

- BuiltIn：使用Nature 内置的转换器进行转换。

  

# converter

`converter` 用于实现 `Meta` 间 `Instance` 的转换，一般需要自己实现，Nature 也有内建及自动化的 `converter` 实现。

[Write a local-converter](howto_localRustConverter.md)

调用的接口形式：





## 如何实现一个 `converter`

`converter` 是面向业务的，没有技术上的难点，`converter`会接收一个``类型的输入 you will only concern about one input-parameter : `meta`'s `instance` and generate one or more output `instance`s

## static converter (Static Orchestration)

Converter Configuration must be added to `relation` table, so that it can be loaded before process `instance`s .In this way the  `relation` can be cached so it's efficient, 

## dynamic-converter (Dynamic Orchestration)

You can dispatch you task at runtime for any downstream `meta` undefined. In this way you need provide `converter` in every inputted `instance`, It would spend more time than `Static Orchestration`, but it's flexible.

__Notice__ dynamic-meta can only use dynamic-converter and only can generate dynamic-meta (see [Meta](meta.md)).



```
LOCALRUST|HTTP|HTTPS
```

### Protocol example

- Http

```json
{
    "protocol":"Http",
    "url":"http://some_domain:8081/some_converter"
}
```

- LocalRust

```json
{
    "protocol":"LocalRust",
    "url":"some_lib.dll:some_converter"
}
```

## Batch process

can set finished context when finished stored.

