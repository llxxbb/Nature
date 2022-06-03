# 接收订单

我们假设已经有了一个商城系统，用户在这个系统里可以选购商品并提交订单。现在我们要借助 Nature 来接管订单的后续处理过程。

**Q**:是否可以将选购商品等这些商城的职能用 Nature 来实现？

**A**:Nature 目前倾向于后端处理，没有前端交互能力，但可以为前端提供数据，即使是提供数据，现在功能上还不完备，如支持缓存等。

## 外系统提交订单

我们需要用 json 格式来提交订单数据，并将数据提交到`http://localhost:8080/input`。如果成功该接口则会返回一个`instance`实例的ID。提交的具体方式请参考

> nature-demo::emall::emall_test()

json 格式示例如下：

```rust
{"data":{"meta":"B:sale/order:1","content":"please fill this property with real order data..."}}
```

- `data.meta=“B:sale/order:1”`：说明这是一个订单数据，`Meta` 必须事先在 Nature 中注册才能使用，下面会讲怎么注册。
- `data.content="..."`则是订单的实际内容。

## 在Nature里注册`Meta`：订单

要想让Nature 接受 上面的订单信息输入，我们需要向 meta 数据表里插入下面的数据：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/order', 'order', 1, '', '', '');
```

**注**：本demo中所用的sql 都可以再 [demo-emall.sql](doc/demo-emall.sql) 中找到，其它 demo 也都有对应的 sql 文件。

我们逐一解释一下：

- meta_type='B': 为`Meta`指定类型B，B指的是`MetaType::Business`，代表这是一个业务对象，其他类型可参考[meta.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/meta.md)
- meta_key='sale/order' : 为`Meta`的名字，用于区别其它业务对象的定义。
- description=‘order’：向别人介绍一下这个`Meta`是干什么的，意义是什么等。
- version=1: 当前业务对象定义的版本号。
- **Nature 要点**：每当业务定义发生变更时，可以插入更高版本的业务定义，而不是更新原有的业务定义。这种做法的好处是可以使业务平滑过渡，而不用担心上一版本正在处理中的数据受到影响，**Nature 为业务系统的迭代提供了良好的内在支持**。

## 查看输入的数据

让我们先看看 Nature 所插入数据的样子。运行：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

打开 instance 数据表，我们会发现有下面的数据：

| ins_key                                           | content                                                                                                                                                              |
| ------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| B:sale/order:1\|3827f37003127855b32ea022daa04cd\| | {"user_id":123,"price":1000,"items":[{"item":{"id":1,"name":"phone","price":800},"num":1},{"item":{"id":2,"name":"battery","price":100},"num":2}],"address":"a.b.c"} |

- “B:sale/order:1” 实际上就是**”meta_type:meta_key:version“**的值的表现形式，Nature 称之为 `meta_string`。
- ins_key：用于唯一标记此条数据。其构成为 “meta_string|id|para”。此例中我们没有输入id，Nature会用输入数据的 hash 值来作为此条数据的 ID，这样做的目的是为了追求**幂等**。此例中我们也没有输入 para ，所以此条数据尾巴上只有一个“|”
- **Nature 要点**：之所以不省去看似“无意义”的“|”是为了便于进行 like 数据检索时有一个明确的休止符。
- content 是我们模拟的订单数据，这个数据是 emall_test() 给出的，大家可以自行去看源码。

## 定义订单状态

先结束 nature.exe 的运行，我们继续完善我们的示例。

这个示例的要点就是要跟踪订单的处理状态。状态数据是不建议直接放到`B:sale/order:1`上的。

- **Nature 要点**：Nature 会为每次状态变更单独记录一个版本，如果订单状态与订单合并，就会有很多的冗余，性能也好不到那里去。所以Nature 是非常提倡将基本信息和状态信息分成两个 `Meta` 这种做法的。
- **Nature 要点**：独立的状态数据会更有利于流程梳理，可以使我们非常直观的、严格的将数据分成有状态的和无状态的，而不是一个混合体从而导致业务概念混乱不清。

为此我们需要为订单状态单独创建一个`Meta`:

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/orderState', 'order state', 1, 'new|paid|package|outbound|dispatching|signed|canceling|canceled', '', '{"master":"B:sale/order:1"}');
```

- states='new|paid|package|outbound|dispatching|signed|canceling|canceled': 这里定义了我们订单里要用的的状态。“|”说明这些状态**不能共存**，同一时间里只能是其中的一个。具体语法请参考：[使用meta](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/meta.md)。
- master="B:sale/order:1"：说明 orderState 依附于订单。其作用有两个：
  - orderState  会使用 订单的ID作为自己的ID
  - orderState 作为上游驱动下游数据时，Nature 会顺便将 order 数据传递给下游，这样下游就不需要单独再查询一次订单数据了。

## 定义`订单`和`订单状态`之间的关系

要想生成订单状态数据，我们需要建立起订单和订单状态之间的`关系`。请执行下面的sql：

```mysql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:sale/orderState:1', '{"target":{"state_add":["new"]}}');
```

| 字段或属性     | 说明                                                                                                                                                     |
| --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| from_meta | `关系`的起点，为 meta_string                                                                                                                                  |
| to_meta   | `关系`的终点，为 meta_string                                                                                                                                  |
| settings  | 是一个 `JSON` 形式的配置对象，用于对这个`关系`进行一些附加控制，如`执行器`，`过滤器`以及对上下游实例的一些要求等。请参考[使用 Relation](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md) |

- **Nature 要点**：在Nature `关系`是有方向的，这个方向说明了数据的流转方向，上面的`关系`定义说明了数据只能从 B:sale/order:1 流向 B:sale/orderState:1。
- target.state_add=["new"]：是说在新生成的数据实例（B:sale/orderState:1）上附加上”new“ 状态。这个语法是数组，也就是说我们可以同时附加多个状态。
- **Nature 要点**：这个附加是在上一个版本的状态基础上进行附加的。对于本例来讲上一版本还不存在，则认为上一状态为“[]”。

## 运行 Demo 并查看生成的订单状态数据

让我们见证一个**魔法时刻**，运行：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

打开 instance 数据表，我们会发现有一条下面的数据：

| ins_key                                                | states  | state_version | from_key                                          |
| ------------------------------------------------------ | ------- | ------------- | ------------------------------------------------- |
| B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\| | ["new"] | 1             | B:sale/order:1\|3827f37003127855b32ea022daa04cd\| |

我们只 在`meta` 和 `relation`数据表里加了两条配置数据，神奇的是`instance`数据表里自动生成了一条“sale/orderState”数据。

- **Nature 要点**：当`关系`中的下游`Instance.content`没有意义时，我们就不需要一个明确的`执行器`来完成`关系`所要求的数据转换任务，在此种情况下Nature 会为`关系`自动生成一个类型为`auto`的`执行器`，正是这个`执行器`帮助我们生成了上面这条数据。 有关使用`执行器`的例子，在后续的章节中会讲到。
- **Nature 要点**：传统编程方式下，对状态数据进行编码是一项无法避免的工作，但 Nature 可以替你完成这件工作，使程序员把精力放到真正需要的地方。
- **Nature 要点**：将订单数据和状态数据分开存储，相较于传统方式的合并存储，看似复杂化了设计，但对 Nature 来讲这却是规范性的设计，这种规范性有利于 Nature 为你简化代码实现的复杂度，如本例所看到的，Nature 可以为你自动生成状态数据并操作状态；如果是合在一起，那么有些事情就需要程序员自己来处理了。

如果仔细看，你会发现上面这条数据的`ins_key` 和 `from_key` 中的 ID 是相同的，这是`Meta.master`设置在起作用。

* **Nature 要点**：master 属性既规范了业务描述又简化了开发。
* **Nature 要点** ：在 Nature 里多个不同元数据实例共享相同的 ID 是一种推荐的做法，这个ID 可以被视为一个**事务ID**。这样**用一个ID就可以把所有相关的数据提取出来**。这要比依赖于外键的传统数据表提取数据有效率的多，而且还减少了关系数据的维护。更重要的是这种处理方式**减少了保障数据一致性的技术复杂度**。
* **Nature 要点**：`from_key` 是 Nature 自动添加的，可用于**追溯数据**，这会为排查问题提供极大的方便。

同时我们发现`target.state_add=["new"]`也发挥了作用：这条数据的`states`被设置成`["new"]`了。

- **Nature 要点**：对于状态处理我认为是传统编程方式下最复杂、最容易出错和最难维护的部分之一，而 Nature 为此提供了一整套处理机制，程序员基本上无需干预就可以处理好所有状态相关的问题，这在后续demo中会经常得到体现。

在本示例的源码中，我们多次提交了相同的订单数据，Nature 会返回相同的ID，也就是说 **Nature 是幂等的**。

- **Nature 要点**：幂等是 Nature 设计的一个重要原则，是保障**数据一致性**以及**失败任务可以重试**的重要机制。
