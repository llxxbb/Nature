# Nature 应用示例

[English](README_EN.md)|中文

如果你是第一次了解 Nature , 建议你从头到尾阅读这些 Demo。 每个章节都包含一些不同的 **Nature 要点**，以说明如何用 Nature 独有的方式来解决问题。

为了能够运行本示例程序，请先进行一些[准备工作](doc/ZH/prepare.md)

## 网上商城订单处理

这个Demo涉及的场景比较多，如订单，支付，库房，配送以及签收等。这不是一个具有生产力的示例，但却简练的勾勒出系统的骨架以及她所具有的强大的支撑及扩展能力。

| 章节                                                                 | 内容摘要                                                                             | Nature 要点                                                    |
| ------------------------------------------------------------------ | -------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| [接收订单](doc/ZH/emall/emall-1-order-generate.md)                     | 大致讲解一下Nature的使用方式，介绍Nature的一个重要的能力：有些业务只需要配置一下不需要代码就能**自动完成**。                   | `Meta`, master, target-state, 自动执行器，状态数据与非状态数据，关系，事务ID，追溯，幂等 |
| [订单账](doc/ZH/emall/emall-2-order-account.md)                       | 将外部逻辑编织到 Nature 中的能力。**消灭代码中的控制逻辑**，体现 Nature 的主导和规范能力。                          | localRust 执行器，数据一致性，自上而下的控制与自下而上的选择。                         |
| [支付订单](doc/ZH/emall/emall-3-pay-the-bill.md)                       | 我们只写了很少的业务代码，就实现了支持多次支付的复杂场景，Nature 会在幕后提供很多保障，如**数据一致性**，**并发及冲突**等问题。          | 系统上下文，并发冲突，状态数据处理。用状态选择控制流程，数据追溯                             |
| [出库](doc/ZH/emall/emall-4-stock-out.md)                            | 如何与涉及到人工和（或）机械设备的**慢系统**或**遗留资产**打交道。                                            | 回调，http执行器，外部提交状态数据。MetaType::Null                           |
| [配送](doc/ZH/emall/emall-5-delivery.md)                             | 这里展示了如何**记录第三方数据**的方法，便于利用这些数据与第三方系统结算。另外Nature 提供了一种机制，用于主干流程被其它一业务中断后再连接起来的情景。 | 参数化输入, id_bridge                                             |
| [签收](doc/ZH/emall/emall-6-signed.md)                               | 利用 Nature 的 retry 可以完成需要特定时间运行的任务                                                | 延迟处理                                                         |
| [附录-多个库房](doc/ZH/emall/emall-appendix-multi-warehouse.md)          | 利用**上下文**将订单分配到不同的库房生产。                                                          | 自定义上下文，上下文选择控制流程                                             |
| [附录：多级配送中转](doc/ZH/emall/emall-appendix-multi-transfer-station.md) | 非编程方式处理业务上的**循环**结构。                                                             | 选择器的组合使用。use_upstream_id， append_para                        |

## 学习成绩统计

Nature 不但可以搞定复杂的业务流程，也可以搞定流式计算。即使你不了解 hadoop,hive,spark等框架也可以玩得转大数据。这个也许不是性能最好的，但我想是生产力非常高的一个。

下面给出一个成绩统计的例子：

| 章节                                                         | 内容摘要                                     | Nature 要点                   |
| ---------------------------------------------------------- | ---------------------------------------- | --------------------------- |
| [全员成绩单->个人成绩](doc/ZH/score/score_1_to_persion.md)          | 使用 Nature 的内置执行器 scatter 来实现成绩单的拆分       | scatter，后置过滤器               |
| [求出每个人所有科目的总分](doc/ZH/score/score_2_person_total_score.md) | “没有状态”的状态数据，利用状态数据和内置执行器 sum 来完成个人总成绩的统计 | is_state，`para`作为选择条件，merge |

## 销售统计

上一个例子不太适合于大并发下的即时统计，像电商类的即时销量 top 统计。现在让我们用一种新方式尝试一下。

| 章节                                | 内容摘要                                                            | Nature 要点                                             |
| --------------------------------- | --------------------------------------------------------------- | ----------------------------------------------------- |
| [订单拆分](doc/ZH/sale/sale_1.md)     | 我们将统计每个商品的销量和销售额，并通过应用 MetaType::Multi 来提升性能                    | MetaType::Multi                                       |
| [定义统计时间区间](doc/ZH/sale/sale_2.md) | 使用区间统计技术可以避免基于状态数据的统计，出于演示的目的，这里以秒我单位进行演示。这里涉及到几个 Nature 的高级用法。 | cache_saved, time_range， append_para，para.dynamic     |
| [单品销售额统计](doc/ZH/sale/sale_3.md)  | 对秒区间内的单品进行销售额自动统计。                                              | convert_before, instance-loader, delay_on_para, merge |
| [销售额top](doc/ZH/sale/sale_4.md)   | 这里涉及到应用上的几个重要技巧。                                                | 任务归一化，MetaType::Loop,task-checker                     |
