# 使用Meta

## Meta 的构成

`Meta` 由三部分构成：`MetaType`， `key` 和 `version` ,可以用一个由 ":" 分隔的字符串来表示。如用“B:sale/order:1” 来表示一个“订单”的 `Meta`，其中 ：

- “B” 为 `MetaType`
- “sale/order” 为`key`
- “1” 为`version`。

## MetaType

下面的表格是 Nature 所支持的 `MetaType`.

| **类型**           | **说明**                                                     |
| ------------------ | ------------------------------------------------------------ |
| (**B**)usiness     | 表示一个**业务**对象，用B表示。该类型的 `Meta` 必须在 `meta` 数据表里定义才能使用。 |
| (**S**)ystem       | 表示一个**系统**对象，由 Nature 自行管理，无需再`meta` 数据表里定义。 |
| (**D**)ynamic      | 用于表示一个**运行时**由外部**动态**定义的**业务**对象，无须在 `meta` 数据表里定义。 |
| (**N**)ull         | 表示一个没有实际意义的对象，用于`converter` 无输出的情况。无须在 `meta` 数据表里定义。 |
| (**M**)ulti-Target | 允许 `converter` 返回多个不同类型的业务对象，每个业务对象需要在`MetaSetting` 中进行定义。该类型的 `Meta` 必须在 `meta` 数据表里定义才能使用。 |

## key

key  用于唯一区别不用的业务对象，其中的**"/"**用于业务域的层级划分，对业务领域的可视化管理非常有帮助。

## version

一个业务对象随着业务的发展可能会发生变化，为了不影响已有的功能，Nature 使用不同的 `version` 来重新定义一个**“新的业务对象”**，但又保持了它们的**业务领域**的一致性，既它们拥有相同的 `key`。

## State

一个业务对象可以有状态，如一个订单可以有下面的状态：

```
new, paid, picked, outbound, delivering, signed
```

每当 `instance`  的状态发生变更时，之前的状态并不会消失掉，Nature 会增加一个版本号用于保存新的状态数据。既每个`instance`有自己独立的 一套`state-version`，相互间不受影响，

Nature 的状态表现形式非常强大，可以构建非常复杂的状态，如分组和排它，分组里面还可以再嵌套分组和排它，排它里面嵌套分组等。

举一个例子，假设我们有 s1，s2，s3，s4四个状态，s1又包含s1-1，s1-2，s1-3，s1-4四个子状态，s1-3和s1-4是互斥的，s2和s3也是互斥的，则我们可以用下面的字符串表达式来表示这个复杂的状态:

```
s1[s1-1,s1-2,s1-3|s1-4],s2|s3,s4
```

表达式中所用到的符号定义如下：

| **符号** | **功能描述**                                                 |
| -------- | ------------------------------------------------------------ |
| ,        | 用于分隔不同的状态                                           |
| [,]      | 表示一个状态分组，分组内的状态也用 "," 分隔                  |
| \|       | 使用“\|”链接的状态是互斥的，一个`instance`只能允许其中的一个状态存在。 |

**重要提示：** 

- 表达式中每个状态的名字必须唯一，即使位于不同的分组中。
- 如果`Meta`的 `state` 属性为空，那么这个`Meta` 将是非状态的。除非 `config` 属性中明确指定： “is_state”:true

## Meta Settings

`Meta` 的 `config` 属性实际上是 `MetaSetting` 的 JSON 形式。

```rust
pub struct MetaSetting {
    pub is_state: bool,
    pub master: Option<String>,
    pub multi_meta: Option<MultiMetaSetting>,
    pub conflict_avoid: bool,
}
```

- is_state:  缺省为 false，适用于`Meta`的 `state` 属性为空但又需要状态功能的时候，可以将这个属性设置为true,。如一个计数器 `Meta` 是需要状态的.

- master: 缺省为 None，当前`Meta`依附于指定的`Meta`。当前`Meta`的`instance`会使用`master`对应`instance`的ID。如果 `converter` 的输入是当前 `Meta`, 则 Nature 会将其对应 `master` 的 `instance` 也一并传入。这也是 Nature 实现自动 `converter` 魔法的依据。注意：如果 [`Relation`](relation.md) 的配置中使用了 `use_upstream_id` ，则优先使用 上游 `Instance`的ID。

- multi_meta：缺省为 None，意味着 `converter`将返回多个不同的`Meta`实例，如根基一个输入数据进行多维度统计。这可以减少大量的 `Meta` 定义和 `converter` 定义。子`Meta`的定义由`MultiMetaSetting`结构进行说明：

  ```rust
  pub struct MultiMetaSetting {
      pub prefix: String,
      pub version: u32,
      pub keys: Vec<String>,
      pub meta_type: MetaType,
  }
  ```

  每个子 `Meta` 由 `meta_type` + `prefix` + `keys`[i] + `version `来进行定义， 如果 `prefix` 的值为空则用父 `Meta`的 `key` 替代。

- conflict_avoid：缺省为 false。想象一个订单销售情况统计的场景，我们希望每分钟有一个统计任务`instance`，而每个新增订单都会产生一个统计任务，这会导致大量重复的`instance`，性能会严重性能。此时我们可以将统计任务的`Meta`的`conflict_avoid`设置为true，Nature 便会记住已经保存的统计任务一段时间，从而避免了资源的浪费，并提升了处理性能。

## 保存 `Meta`

## Store `Meta` data

下面为“订单” `Meta` 的示例：

```sqlite
INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('B:sale/order', 'order', 1, '', '', '{}');
```

full_key由“{`MetaType`}:{`key`}”构成。

## 限制说明

Nature 不允许 `converter` 返回多个具有不同 `Meta` 的实例，除非目标 `MetaType` 为”M“。

如果 `Meta` 是有状态的，那么 `converter` 只能返回一个 `instance`。这是因为 Nature 对于状态数据的冲突处理较为复杂，需要 "re-convert"，如果是返回多个状态数据，多个状态数据的一致性将非常难以保证。