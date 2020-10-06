# Meta

`Meta`用于定义业务数据，既是业务数据的设计时， 业务数据的运行时为 `Instance`

## Meta-String

一个完整的 `Meta` 由三个部分构成：`MetaType`， `key` 和 `version` 。为了简单起见 Nature 用一个字符串来表示，其形式为:

```
MetaType:key:version
```

`Meta-String` 会用在诸如 `Instance.meta` 等属性上。如“订单”实例的 `Meta-String` 可以表示成“B:sale/order:1”。

### MetaType

Nature 所支持的 `MetaType` 列表如下：

| **类型**           | **说明**                                                     |
| ------------------ | ------------------------------------------------------------ |
| (**B**)usiness     | 表示一个**业务**对象。该类型的 `Meta` 必须在 `meta` 数据表里定义才能使用。 |
| (**S**)ystem       | 表示一个**系统**对象，由 Nature 自行管理，无需再`meta` 数据表里定义。 |
| (**D**)ynamic      | 用于表示一个**运行时**由外部**动态**定义的**业务**对象，无须在 `meta` 数据表里定义。 |
| (**N**)ull         | 表示一个没有实际意义的对象，用于`Executor` 无输出的情况。无须在 `meta` 数据表里定义。 |
| (**M**)ulti-Target | 一般情况下 Nature 不允许 `Executor` 返回多个具有不同 `meta` 属性的 `Instance`，除非使用此种类型的 `MetaType` 。此类型要求在设置中定义`multi_meta`属性，请见下面的说明。该类型的 `Meta` 必须在 `meta` 数据表里定义才能使用。 |
| (**L**)oop         | 重复创建自身，直到 sys_context 含有 FINISHED 属性。一般用于批量处理。该类型的 `Meta` 必须在 `meta` 数据表里定义才能使用。 |

### key

key  用于唯一区别于其它业务对象，可以用"/"进行业务域的层级划分，这会对业务领域的可视化管理非常有帮助。

### version

一个业务对象随着业务的发展可能会发生变化，为了不影响已有的功能，Nature 使用不同的 `version` 来重新定义一个**“新的业务对象”**，但又保持了它们的**业务领域**的一致性，既它们拥有相同的 `key`。所以 `version` 对业务的扩展会非常有帮助。

## State

一个业务对象可以有状态，如一个订单可以有下面的状态：

```
new, paid, picked, outbound, delivering, signed
```

每当 `instance`  的状态发生变更时，Nature 并不会覆盖之前的状态，而是增加一个新的 `Instance`，其状态版本号在之前的基础上加1。

Nature 的状态定义的表现形式非常强大，可以构建非常复杂的状态，如分组和排它，分组里面还可以再嵌套分组和排它，排它里面嵌套分组等。

举一个例子，假设我们有 s1，s2，s3，s4四个状态，s1又包含s1-1，s1-2，s1-3，s1-4四个子状态，s1-3和s1-4是互斥的，s2和s3也是互斥的，则我们可以用下面的字符串表达式来表示这个复杂的状态:

```
s1[s1-1,s1-2,s1-3|s1-4],s2|s3,s4
```

表达式中所用到的符号定义如下：

| **符号** | **功能描述**                                                 |
| -------- | ------------------------------------------------------------ |
| ,        | 用于分隔不同的状态                                           |
| [,]      | 表示一个状态分组，分组内的状态也用 "," 分隔                  |
| \|       | 表示状态是互斥的，在 `Instance` 中只允许其中的一个状态存在。 |

**重要提示：** 

- 表达式中每个状态的**名字必须唯一**，即便是位于不同的分组中。
- 如果`Meta`的 `state` **属性为空，那么这个`Meta` 将是非状态的**。除非在 `Meta` 的设置中明确指定： “is_state” 为 true

## Meta 设置

`Meta` 的设置信息为 JSON 形式，其定义如下：

```rust
{
    "is_state": false,		// 缺省false, 如果`Meta`的 `state` 属性为空但又需要成为状态数据时，可以将这个属性设置为true。如一个计数器 `Meta` 是需要状态的。
    "master": null,			// 缺省null，见下面的说明
    "multi_meta": [],		// 缺省null，见下面的说明
    "cache_saved": false,	// 缺省false，见下面的说明
    "only_one": false,		// 缺省false, 见下面的说明
}
```

- master: 为指向另一个 `Meta` 的 `Meta-String` 。master 所对应的 `Instance` 有几个作用：一是作用 `master` 属性传递给 `Executor`, 这点对于业务的基本信息与状态信息分离是非常便利的，如[示例](https://github.com/llxxbb/Nature-Demo)里的订单与订单状态数据的分离。 二是其id会作为当前`instance`的id。 三是 Nature 实现自动 `Executor` 魔法的依据。注意：如果 [`Relation`](relation.md) 的设置中使用了 `use_upstream_id` ，则优先使用 上游 `Instance`的id。
- multi_meta： 为 `Meta-String` 数组。`MetaType` 为 M 的 `Meta` 可以允许`Executor` 返回多个不同`Meta`的`Instance`。而这些用于返回的 `Meta` 必须在这里定义，且也必须作为独立的 `Meta` 进行定义。**注意**：`multi_meta` 不能含有状态 `Meta`。**注意**：如果`multi_meta` 只有一个值（一般常见于MetaType为L的`Meta`），则`Executor` 无需明确给出出参 `Instance` 的 `meta` 属性， Nature会自动填充；如果`multi_meta` 多于一个值，则 `Executor` 的出参必须明确给出 `Instance.meta` 的值。
- cache_saved：为 true 则将生成的 `Instance` 缓存一小段时间，用于避免重复写库以提升效率。常见于不同上游生成相同下游的情况，如[示例](https://github.com/llxxbb/Nature-Demo)中的生成的定时统计任务`Instance`。**危险提醒**：错误地使用此选项可能会消耗大量内存，甚至溢出！缓存时间由 `.env` 文件中的 `CACHE_SAVED_TIME` 选项指定。
- only_one：只对`MetaType` 为 L 的 `Meta` 有效，用于标记 Loop 是否只有一个下游 `Instance` 输出。如果为 false，则 Loop 的每次调用都可以生成多个不同 Meta 的 `Instance`, 而这些 Meta 由 `multi_meta` 属性给出。 如果为 true ，Nature 则视当前定义的 `Meta` 为一个状态 `Meta`，用于 Loop 每次调用时存放状态数据(内容为 `multi_meta` 指定的 `Meta` 对应的 `Instance` ) 以服务于下次 Loop，注意此种情况下`multi_meta` 只能定义一个元素，之所以用这种方式处理是因为：
  - `multi_meta`  不能接受状态数据，因为同时处理多个状态数据在架构支持上极其复杂。
  - 从用户角度来看用户并不期待 Loop 的中间结果，所以 `multi_meta` 里没有必要是状态数据。

## 定义 `Meta`

`Meta`数据时存放到 meta 数据表中的，下面为“订单” `Meta` 的示例：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/order', 'order', 1, '', '', '');
```

## 限制说明

如果 `Meta` 是有状态的，那么 `Executor` 只能返回一个 `instance`。这是因为 Nature 对于状态数据的冲突处理较为复杂，很难保证状态数据的一致性。