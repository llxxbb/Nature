# 配送

接下来我们需要一些快递公司来帮助我们将包裹送给消费者，Nature 将记录这些派件单信息并在以后的某个时间进行查询，如每个月的结算。

我们想按照快递公司名称和派件单ID来与对方进行结算，假设我们不想在Nature 外单独建立一个数据库来存储这些信息，让我们看一下Nature 是怎么面对这个问题的。

## 记录`派送单`信息

首先我们来定义一下`派送单`信息，用于日后的结算：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'third/waybill', 'waybill', 1, '', '', '');
```

因为快递公司是直接来人取件，所以快递公司名称和派件单ID等信息需要在出库时记录到上一节中提到的库房系统中。我们可以设计一个`订单出库状态 -> 派件单`的`关系`来将这些信息提取出来并形成派件单信息。`关系`定义如下：

```mysql
-- orderState:outbound --> waybill
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:third/waybill:1', '{"id_bridge":true, "selector":{"state_all":["outbound"]}, "executor":{"protocol":"localRust","url":"nature_demo:go_express"}}');
```

我们看到了一个新的属性：`id_bridge`，其作用在稍后讲，这里先忽略一下。有关选择器的使用在[支付订单](emall-3-pay-the-bill.md)中已经介绍过，请参考 [meta.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/meta.md)。

`执行器`的具体实现方式请参考对应的源代码，这里有一点需要说明一下：

- **设置`Instance.para`属性**：用于记录派件单相关信息，其形式为：“/[快递公司ID]/[派件单ID]”。**参数之间请务必用“/”进行分隔**（你可以通过改变 Nature 的启动参数来将它变成其它字符）。

让我们看一下运行结果，运行：

```shell
nature.exe
nature_demo_executor_restful.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

结束后我们会发现有下面的数据产生：

| ins_key                                                    | from_key                                                  | sys_context                                     |
| ---------------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------- |
| B:third/waybill:1\|0\|/ems/3827f37003127855b32ea022daa04cd | B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\|\|4 | {"target.id":"3827f37003127855b32ea022daa04cd"} |

- **Nature 要点**：你会发现`派件单`的ID为0，Nature并没有为这个`instance` **Hash**出一个值来。这样做的原因是因为我们指定了 `para`，这会让 Nature 认为这是一条外部数据。如果 Nature 对这个ID进行了填充，当检索/ems/开头的派件单数据时将会是一件非常麻烦和低效的事。当然 Nature 并不阻止你自行填充这个 ID 值。
- **Nature 要点**：Nature 提供的检索能力是有限度的，毕竟 Nature 的主要目的不是用来检索数据而是用来处理数据的。

## 将订单的状态置为“配送中”

货物已经在路上了，此时应当将订单的状态更新为配送中。


```mysql
-- waybill --> orderState:dispatching
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:third/waybill:1', 'B:sale/orderState:1', '{"target":{"state_add":["dispatching"]}}');
```

很高兴，再一次不需要写代码就可以完成任务。让我们看一下运行结果，运行：

```shell
nature.exe
nature_demo_executor_restful.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

结束后我们会发现有下面的数据产生：

| ins_key                                                    | states          | state_version | from_key                                                  |
| ---------------------------------------------------------- | --------------- | ------------- | --------------------------------------------------------- |
| B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\| | ["dispatching"] | 5             | B:third/waybill:1\|0\|/ems/3827f37003127855b32ea022daa04cd\|0 |

这里有个问题，派件单是如何知道要更新哪一个订单的状态的？细心的读者可能已经注意到`派件单`数据的`sys_context`里的`target.id`存放的就是订单的ID，那么派件单里的这个ID又是从哪里来的呢？如果你去看 go_express 的源代码，你会发现我们并没有设置`Instance.sys_context`属性。不卖关子，写这个属性的是 Nature 自已，还想着我们上面提到的 `id_bridge` 吗：

- **Nature 要点**：派件单的两端都是订单状态，而派件单的ID不使用订单的ID，这就中断了ID的传递，而`id_bridge` 在派件单上方架起了一座桥梁，使得ID可以被传递，而传递的意义在于：实施人员只需关注领域内的事情，无需关心领域间协作的问题。最直接的体现是**你可以避免写代码或少些代码**，如本小节中我们就不需要写代码。
- **Nature 要点**：`id_bridge` 可以跨越多个节点进行搭桥，但要求中间的所有节点都需要指定 `id_bridge`  或在`Instance.sys_context`中指定 `target.id`。

