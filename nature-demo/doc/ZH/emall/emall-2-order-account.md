# 建立支持多次支付的订单账

 ## 为订单建立账户

为了能够使一个订单能够支持多次支付，我们需要为每一笔订单建立一个独立的账户，来记录应收和实收情况。其`Meta`定义如下：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}');
```

我们可以看到这也是一个状态数据，里面有一组互斥的状态定义。另外它的 `master`也指到了 `order`上。有关这两个点已经在[上一节](emall-1-order-generate.md)中解释过了，这里不再说明。

## 将应收写入订单账

订单信息里含有应收信息，所以我们需要建立订单和订单账之间的关系。

```mysql
-- order --> orderAccount
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"state_add":["unpaid"]}}');
```

我们先认识一下几个新的属性：

| 属性              | 描述                                                         |
| ----------------- | ------------------------------------------------------------ |
| executor          | 用于告诉 Nature 使用用户自定义的转换器                       |
| executor.protocol | 告诉 Nature 如何与 `executor`通讯。`LocalRust` 是本地 lib 包。 |
| executor.url      | 告诉Nature 哪里可以找到这个 `executor`，以及入口是哪个。     |

在这里 Nature 不能再为我们自动创建`orderAccount`实例了，因为 Nature 不知道如何写它的 `content`。这就需要我们借助外部来实现了，为此我们引入了新的 配置项：`executor`。`executor`方法的入参、出参请参考 [reladtion.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)，方法的具体实现请自行查看示例代码。

这个方法的主要作用就是从订单（入参）中提取应收数据，并将应收写入到出参实例的 `content`中。

- **Nature 要点**：在示例代码中我们无需自己从数据库中检索出订单数据，它已经被 Nature 放到入参里了，相较于传统开发方式，我们可能减少了一次查库操作，别看是一次，当大并发和业务链路较长时，性能的提升将是非常可观的。
- **Nature 要点**：Nature 将编程任务进行了恰当的最小力度的分解，使实施者能够快速聚焦和实施，这非常有利于快速迭代。

- **Nature 要点**：Nature 以非编程方式主导了业务流程，并对可编程的范围进行了强制规范和约束。只有特定的入参才可以触发示例代码，且示例代码只可以返回特定的出参，相较于传统方式将会极大改善业务控制系统的能力，杜绝业务系统深陷技术旋涡而不能自拔的现象发生。

- **Nature 要点**：也许你没有发现，我们无意间解决了一个非常复杂的问题。在传统开发方式下，生成`order`的时候一般会同时生成`orderState`和`orderAccount`，并用**数据库事务**来保证一致性。这是自上而下控制的一种常规操作，同时现有数据库事务的处理方式也必须将这三者耦合在一起。而Nature 利用`自由选择`上游的方式实现了即插即用的模块化机制；在本例里没有对已有的订单设计做任何改动，就非常容易的增加了`orderState`和 `orderAccount` ，我们不用担心一致性问题，Nature 会兜底。这一点的改变意义重大：**消灭了代码中的控制器**，每个模块只做好自己就好，协调的事交给 Nature 来做就可以了，相较于传统开发方式，这将极大简化系统复杂度，减少代码量，系统越大效果越明显。

## 运行 demo

请将本例对应的 nature_demo.dll 放入到包含 nature.exe的目录中，运行：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

运行完成后我们就可以在 instance 数据表里看到下面新生成的订单账数据：

| ins_key                                                     | content                                                      | states     | state_version | from_key                                             |
| ----------------------------------------------------------- | ------------------------------------------------------------ | ---------- | ------------- | ---------------------------------------------------- |
| B:finance/orderAccount:1\|3827f37003127855b32ea022daa04cd\| | {"receivable":1000,"total_paid":0,"last_paid":0,"reason":"NewOrder","diff":-1000} | ["unpaid"] | 1             | B:sale/order:1\|3827f37003127855b32ea022daa04cd\|\|0 |

