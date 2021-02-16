# 对订单进行支付

有了订单账和应收信息，我们就可以交费了。为了能更多的演示 Nature 的特性，让我们故意虚构一些复杂的情景。我们假设用户的每张银行卡里的钱都不足以全额支付这笔订单，但是三张卡加起来是可以的。

## 记录每笔支付数据

我们需要支付系统告诉 Nature 用户支付的每一笔费用，为此我们需要定义一个支付单 `Meta`:

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'finance/payment', 'order payment', 1, '', '', '');
```

定义了 `Meta` 后我们就可以向 Nature 输入数据了。输入数据的代码请参考：nature-demo::emall::finance::pay。需要说明的是我们这里用到了`Instance.sys_context`属性，如下：

```rust
sys_context.insert("target.id".to_string(), format!("{}", id));
```

我们在里面放置了一个 `target.id`，其值为**16进制**的订单ID。其作用我们稍后讲。先让我们来看一下demo的运行效果。运行：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

之后我们便可以在 instance 数据表里的看到下面的数据：

| ins_key                                                 | content                                                      | sys_context                                     |
| ------------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------- |
| B:finance/payment:1\|85fcf20d28c053ac2d3103d1759cf123\| | {"order":4665262802592301254545277299928466637,"from_account":"b","paid":200,"pay_time":1589670980281} | {"target.id":"3827f37003127855b32ea022daa04cd"} |
| B:finance/payment:1\|df0d1867b9564ab3963dd8546aefec38\| | {"order":4665262802592301254545277299928466637,"from_account":"c","paid":700,"pay_time":1589670980286} | {"target.id":"3827f37003127855b32ea022daa04cd"} |
| B:finance/payment:1\|e18330eb534abe924a3d03760df3e90c\| | {"order":4665262802592301254545277299928466637,"from_account":"a","paid":100,"pay_time":1589670980275} | {"target.id":"3827f37003127855b32ea022daa04cd"} |

除了已经接触到的 `ins_key` 和 `content`外，这里有出现了一个 `sys_context` 字段，里面放置了上面我们提到的 `target.id`数据。

在demo 示例代码中，我们故意将第二笔支付重新输入了一遍，以验证我们是否可以少交点钱，结果很好，并没有发生糟糕的事情。

## 将支付数据关联到订单账上

接下来我们就需要将这些支付数据记录到订单账上，来完成订单的支付。在此之前我们需要先建立支付单和订单账的关联关系。

```mysql
-- payment --> orderAccount
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:finance/payment:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:pay_count"}}');
```

关系做好了，但我们如何将这三笔不同的账记到同一个订单账上呢？有人会说支付单记录的订单号不就是订单账的号吗，没错，但 Nature 是不理解 `Instance.content`中的内容的。但 Nature 却可以理解 `Instance.sys_context` 中的内容，所以这就是为什么在里面放置 `target.id` 属性的原因了。有了 `target.id` Nature 就可以找到要操作的订单账了。

- **Nature 要点**：订单账是状态数据，Nature 对状态数据有特殊的处理。在将支付单数据提交`执行器`（`pay_count`）处理前，Nature 便会将`orderAccount` 的上一版本查出来一并给执行器（`pay_count`），而这个查询所需要的ID就来源于上面的 `target.id`。

有关`pay_count`是如何工作的请自行查看示例代码。现在我们可以验证一下效果了。运行：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

之后我们便会发现 instance 数据表产生了下面的数据。

| ins_key                                                     | content                                                      | states      | state_version | from_key                                                   |
| ----------------------------------------------------------- | ------------------------------------------------------------ | ----------- | ------------- | ---------------------------------------------------------- |
| B:finance/orderAccount:1\|3827f37003127855b32ea022daa04cd\| | {"receivable":1000,"total_paid":0,"last_paid":0,"reason":"NewOrder","diff":-1000} | ["unpaid"]  | 1             |B:sale/order:1\|3827f37003127855b32ea022daa04cd\|\|0 |
| B:finance/orderAccount:1\|3827f37003127855b32ea022daa04cd\| | {"receivable":1000,"total_paid":100,"last_paid":100,"reason":"Pay","diff":-900} | ["partial"] | 2             | B:finance/payment:1\|e18330eb534abe924a3d03760df3e90c\|\|0 |
| B:finance/orderAccount:1\|3827f37003127855b32ea022daa04cd\| | {"receivable":1000,"total_paid":300,"last_paid":200,"reason":"Pay","diff":-700} | ["partial"] | 3             | B:finance/payment:1\|85fcf20d28c053ac2d3103d1759cf123\|\|0 |
| B:finance/orderAccount:1\|3827f37003127855b32ea022daa04cd\| | {"receivable":1000,"total_paid":1000,"last_paid":700,"reason":"Pay","diff":0} | ["paid"]    | 4             | B:finance/payment:1\|df0d1867b9564ab3963dd8546aefec38\|\|0 |

同一个ID的`orderAccount`一共有4条数据，第一条是创建订单时产生的，其它3条是支付产生的。在第4笔我们欣喜的发现 states 变为 “paid” 了，我们支付成功了。

- **Nature 要点**：传统处理方式一般是采用update的方式将新状态覆盖到旧状态上，而要跟踪这些变化则需要额外的措施来保障，复杂度较高。而 Nature 通过增加版本号的方式来处理这个问题，**Nature 绝不修改、删除数据**，这样所有的数据，所有的改变都可以非常容易的被**追溯**。
- **Nature 要点**：Nature 对互斥状态支持的很好，你无需先删除一个状态再增加一个状态，如果你输入一个新的状态，Nature 会自动替换掉与之互斥的其它状态。
- **Nature 要点**：如果你查询 Nature 的输出日志，你会看到重复提交的数据被忽略掉了，并不影响结果的正确性。
- **Nature 要点**：我们几乎是同一时间提交了多笔支付数据，你会在 Nature 的输出日志上看到 “conflict” 字眼。这说明 Nature 为你解决了并发冲突问题，避免了脏数据的提交，而这一切对程序员来讲是无感知的。
- **Nature 要点**：Nature 已经向你展示了不同业务事件控制状态的能力，而无需高难度的编程。由上面两个要点来看，Nature 大大降低系统的技术风险，同时也降低了程序员被罚款的风险：），还有借助 Nature 普通程序员可以挑战高级程序员的工作了。

## 是时候设置订单的状态了

订单账已经齐活了，接下来我们要将这个好消息告诉订单（状态），先建立一个关系：

```mysql
-- orderAccount --> orderState
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:finance/orderAccount:1', 'B:sale/orderState:1', '{"selector":{"state_all":["paid"]},"target":{"state_add":["paid"]}}');
```

很高兴这里没有见到`executor`，也就是说我们又可以省去编码工作了，但我们还是要费点脑筋学点新东西。

- selector.state_all: 
  - selector是选择过滤器，只有符合条件的上游数据才可以进行关系处理。
  - state_all 是 selector 的一个条件，意思是上游状态数据必须包含所有我指定的状态。请参考[使用 Relation](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)
- **Nature 要点**：Nature 提供了对上游状态数据进行**选择**的能力，可以通过非编程的方式来精细化控制执行器的输入。

在本示例里只有订单账的第4条数据可以满足这个条件，其它3条都不能满足。这一点可以通过下面的 from_key 来见证，最后面的4就是版本为4的订单账。

| ins_key                                                | states   | state_version | from_key                                             |
| ------------------------------------------------------ | -------- | ------------- | ---------------------------------------------------- |
| B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\| | ["paid"] | 2             |B:finance/orderAccount:1\|3827f37003127855b32ea022daa04cd\|\|4|


## Nature 幕后为你做了什么

- **Nature 要点**：回过头来我们再看一下这一小节的内容，我们从输入支付数据到订单账再到订单状态，我们串接了三个节点，而Nature 可以让你无限度的串接，来满足你庞大的业务体系。
- **Nature 要点**：Nature 不只是将业务点串成线，而且可以多个业务线交织成网，以一种即时可见的方式让你洞察业务布局的合理性。Nature 用足够短和足够通用的`关系`，来构建强大且灵活的业务系统。
- **Nature 要点**：在本章节的Demo示例中我们大约写了100行的代码，完成了这个复杂的业务逻辑。包含并发，状态冲突控制，重试策略等，在传统开发模式下我们需要写多少代码呢？



