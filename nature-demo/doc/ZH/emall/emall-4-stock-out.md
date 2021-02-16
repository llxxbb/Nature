# 出库

现在我们该履行合同了。第一步就是出库，我们假设库房管理系统已经存在，为此我们需要实现一个执行器来与库房系统通讯。但因为库房的拣货、打包涉及到人工和（或）机械设备的处理，时间很长，导致执行器执行超时，无法与Nature 协作，既同步的方式无法满足 Nature 的通讯要求。

**一些限制说明**：

在真实的情况中，一个订单可能包含不同的商品，而这些商品也可能分布在不同的库房中。本示例为了简单起见，假定所有的商品都在同一个库房里。

这里有两种解决方法，第一种方式是建立如下`关系`, 用执行器将 Nature 的订单加工成`出库单`并导入到库房系统。

```mysql
-- orderState:paid --> stockOutApplication
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'N:warehouse/outApplication:1', '{"selector":{"state_all":["paid"]},"executor":{"protocol":"localRust","url":"nature_demo:stock_out_application"}}');
```

- **Nature 要点**：请注意这里的`N:warehouse/outApplication:1`，我们之前并没有定义过，这是一个不存在的`Meta`。 为了简化配置工作，对于没有存储意义的`Meta`不需要定义就可以使用，`出库单`是存储到库房系统里的，没有必要再在 Nature 里存储一份。我们用`N:`来标记这样的`Meta`，N 代表 `MetaType::Null`。`warehouse/outApplication`只是助记符，`N:warehouse/outApplication:1` 完全可以写成`N::1`，后面的版本号无论是多少都会被置为1，因为“空”有很多版本也没有意义。请参考 [meta.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/meta.md)。
- **Nature 要点**：Nature 并不会因为不保存 MetaType::Null 类型的实例数据，而降低服务质量，同样的保障机制会作用在相关的执行器上。

`出库单`进入库房系统后，人员及设备就可以开工了，当打包完成后就需要调用 Nature 的 input 接口来改变订单的状态，以驱动订单后面的流程。但这种方式，少了一些规范性和约束性，因为库房系统必须填写下面的信息如下：

- 目标`Meta`为：`B:sale/orderState:1`
- 将`instance`的状态置为 package 
- 设置状态的版本号

会有下面的问题：

- 这些信息必须通过编程的方式提交，这样程序员就必须要了解订单状态相关的知识，扩大了信息沟通和维护成本。
- 程序员可能会指定不规范的状态版本号，还有就是必须编程应对状态版本冲突的问题。

其实这两个问题都可以避免，这就是我们的第二中方法，也是本示例所采用的方法：利用 Nature 的**回调机制**。

## 订单状态：支付->打包完成

当我们支付完成后，订单状态就会停在`paid`上，直到库房系统给出一个新的状态，所以我们可以定义一个订单状态到订单状态的`关系`。

```mysql
-- orderState:paid --> orderState:package
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:sale/orderState:1', '{"selector":{"state_all":["paid"]},"executor":{"protocol":"http","url":"http://localhost:8082/send_to_warehouse"},"target":{"state_add":["package"]}}');
```

- **Nature 要点**：我们这里看到了一种新的执行器：`http`，借助它 Nature 可以在全球范围内编织一个庞大的系统。

- **Nature 要点**：nature_demo_executor_restful 项目已经提供了对上面url的支持，实现逻辑大家可自行下载源码进行查看。

send_to_warehouse 的实现方式是这样的，将入参直接传递给一个新的线程（实际生产中，你可以采用更好的处理方式）来处理，自己什么也不做并直接返回 下面的结果给 Nature：

```rust
ConverterReturned::Delay(60)
```

这个的意思是说，我要晚会给你（Nature）结果，多晚呢？60秒内。

- **Nature 要点**：Nature 在 `Delay` 指定的时间内不会进行重试。如果不指定 `Delay` Nature 在没有得到响应的情况下，会在接下来的第2、4、8、16、32、64...秒（依据启动参数来确定）进行重试，直到有反馈为止。
- **Nature 要点**：这里的延迟时间是个技术问题，不是业务问题，所以就不放到`关系`里面进行配置了，如果放到那里反而不灵活了。

因为这是个Demo，我们只在50ms便返回了结果。当返回结果时我们不能调用 Nature 的 input 接口了，否则 Nature 挂起的任务会在将来的某个时刻重试。这里应当调用 Nature 的 `callback` 接口，它接受 `DelayedInstances`类型的示例。请注意别忘了把执行器得到的 task_id 给带上，具体请看示例代码。

让我们来看下效果，运行：

```shell
nature.exe
nature_demo_executor_restful.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

结束后我们会发现有下面的数据产生：

| ins_key                                                | states      | state_version | from_key                                                  |
| ------------------------------------------------------ | ----------- | ------------- | --------------------------------------------------------- |
| B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\| | ["package"] | 3             | B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\|\|2 |

## 订单状态：出库

打包对库房来说只是个中间状态，只是为了让顾客及时了解到货物的状态。我们还需要把货物放到出库区，让配送人员将货物拉走。这个`出库`状态也可以走 Nature 回调的路子，实现状态的配置化，但为了演示如何向 Nature 提交状态数据，这里放弃了这种做法，而是直接将状态数据提交到 Nature 的 input 接口，具体请看示例代码。

- **Nature 要点**：一定要设置`instance.id `为要订单的ID，否则Nature 会分配一个新的ID，这将导致订单在系统中无法出库。
- **Nature 要点**：`state_version` 必须要在原有的基础上加一，否则会引起冲突，无法处理。

让我们来看下效果，运行：

```shell
nature.exe
nature_demo_executor_restful.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

结束后我们会发现有下面的数据产生：

| ins_key                                                | states       | state_version | from_key |
| ------------------------------------------------------ | ------------ | ------------- | -------- |
| B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\| | ["outbound"] | 4             |          |

请留意这里的 from_key 为空，这是因为这条数据是外部输入时没有指定这个值，对于这种情况 Nature 不能自动填充这个值。

## 多个库房

这里并不是刻意想着构建一个庞大的电商系统（当然 Nature 有这个能力），只是因为借助多库房来演示 Nature 的一种新技术：上下文选择。如想立马了解可以点击链接：[附录-多个库房](emall-appendix-multi-warehouse.md)



