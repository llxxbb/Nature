# Nature

[English](README_EN.md)|中文

## Nature 是什么？

### 分布式流式计算引擎

Nature 是一个分布式的流式计算引擎。Nature 只强调 map 而不强调 reduce。其核心处理模式为： **数据 --map--> 数据 --map--> 数据...**。这种模式可以让我们更容易聚焦到数据本身上，而不是处理过程上。但这并不妨碍 reduce 的使用，reduce 过程可以在 map 内部来实现。这样就简化和统一了处理模式，而这种模式是 Nature 的架构**核心**所在。

### 业务控制中心

 **数据 --map--> 数据 --map--> 数据**是 **Nature 的运行时模式**，从管理角度来看，我们更关注结果而不是过程，因此 Nature 进一步简化了这个模式，把 map 去掉了，新的模式变成了：**数据 --> 数据 --> 数据...**。这是 **Nature 的设计时模式**。Nature 用 `Meta` 和 `Relation` 来分别表示数据以及数据之间的关系。这样在 Nature 体系内，所有参与协作的业务系统只有 Nature 知道数据所代表的业务是什幺，以及如何通过 `Relation` 来找到下游业务，于是Nature 就成为了实际上的业务控制中心。

### 调度中心

Nature 运行时模式中的 map 对应 `Relation`  中的 `Executor`。Nature 会遵循设计时模式中的数据流向对 `Executor` 进行调度，这些调度包括前置处理，后置处理，幂等，延时处理、定时处理以及批处理等。如果超时或遇到环境异常，Nature 将按照重试策略进行多次重试，以尽最大努力保证数据的最终一致性。即使最终重试失败或遇到逻辑异常，Nature 也并不会丢弃任务，而是将之放入错误数据表。当故障排除后，可以将错误数据表中的数据重新导入到任务数据表，Nature 将会再次重试这些任务直到成功完成。

### 数据中心

Executor 在运行时会生成数据 `Instance`，外系统提交到 Nature 的初始数据也是 `Instance` ，`Instance` 是 `Meta` 的运行时表达，既业务的实例数据。如果您愿意您可以尽可能多得将 `Meta` 交给 Nature 来搭理，Nature 将为这些 `Meta` 所产生的 `Instance` 提供统一的、集中的存储以及提供相应的查询接口，这样 Nature 就扮演了一个数据中心的角色。这里有几点说明：

- 数据检索：Nature 是一种流式作业的处理模式，数据的存取更多的像 Key-Value 数据库，因此不适合作为数据统计的直接数据来源。当然您可以利于 Nature 的流式计算机制加工出任何您想要的数据来，请参考[示例](https://github.com/llxxbb/Nature-Demo)中的销量统计。
- 数据库容量：Nature 缺省使用 mysql 作为后端存储，如果您的数据量很大，可以考虑使用 [Tidb](https://pingcap.com/en/) 。

### 极简开发平台 

您可以从上面的介绍中看到 Nature 整合了很多重量级的元素，而整合的目的就是为了简化我们的编程，使开发者能够更好的聚焦于业务本身。具体体现在以下几个方面：

- 数据驱动

  传统的面向功能的开发会使业务数据，技术数据，控制数据，临时数据，混杂在一起，产生了不必要的耦合，无意中增加了系统的复杂性和维护成本，更为糟糕的是关键业务数据可能被系统“绑架”，使系统变得臃肿、低效且难以改变。而 Nature 的设计时模式完全由数据构成，没有一丁点的功能存在，保证了业务的纯粹性，直观性和简洁性，能够保障对系统的绝对控制权，为整个系统的分工协作、瘦身、优化迭代定下坚实的基调。

- 大幅度减少接口、存储相关的工作

  业务的主体流程控制是通过[配置](doc/ZH/help/relation.md)来实现的，开发者无需关心上下游的流程控制问题。体现在两个方面：一是接口的定义，相较于传统项目群中复杂的、个性化的众多的接口交互定义，Nature 只提供了有限的但通用的几个接口；二是接口的使用，传统项目要在代码里控制好接口的调用关系和顺序，但对 Nature 来讲除了提交数据给 Nature 外，所有的接口调用都是由 Nature 来负责的。这就极大的简化了设计、开发、调试以及后期的维护工作。

  同样的，Nature 采用了集中且统一的数据存储方式，开发者基本上不需要关心数据表、索引的设计以及后续的开发维护工作。

- 大幅度减少非功能性开发的相关工作

  Nature 为系统的可用性（如幂等、数据最终一致性）和可靠性（如重试、异常记录）做了非常多得工作，除此之外，Nature 还在可扩展性上对业务进行了支持，如 `Meta` 的版本技术。依据帕累托法则（二八定律），这些80%的繁重且重要的工作现在都可以由 Nature 来承担。

## 如何使用

我们需要做下面的工作

1. 创建一个[mysql](https://www.mysql.com/) 或 [mariadb](https://mariadb.org/) 或 [Tidb](https://pingcap.com/en/) 数据库，并执行 [schema.sql](shell/schema.sql)

2. 在 [Meta](doc/ZH/help/meta.md) 数据表里定义多个业务对象，如：我们定义`订单`和`订单账`两个业务对象 

   ```sql
   INSERT INTO meta (full_key, description, version, states, fields, config) VALUES
   ('B', 'sale/order', 'order', 1, '', '', ''),
   ('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}'); 
   ```

3. 在 [relation](doc/ZH/help/relation.md) 数据表使定义关系将多个业务对象关联起来（相当于流式计算中的 map），如上面的`订单`和`订单账`可以有这样的定义：

   ```sql
   INSERT INTO relation
   (from_meta, to_meta, settings)
   VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"states":{"add":["unpaid"]}}}');
   ```

4. 请依据`关系`定义中指定的 protocol 实现用于业务对象间转换的逻辑，我们称之为[Executor](doc/ZH/help/executor.md)。如对于`订单`和`订单账`来讲这个逻辑是：

   - 生成一个`订单账`业务对象
   - 从传入的`订单`中提取所有商品的应收款项记为该`订单账`的应收
   - 将`订单账`对象返回给 Nature 以驱动下一环节的处理。

5. 配置好`.env` 文件中的`DATABASE_URL`属性以指向您创建的数据库，

6. 启动 natrue.exe 和 retry.exe。

7. 对 Nature 发起 http post 请求，如将`订单`数据提交数据到 Nature，请参考[Nature接口定义](doc/ZH/help/nature-interface.md)

## 深入了解Nature

[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)

[Nature 架构说明](doc/ZH/help/architecture.md)

[使用 Meta](doc/ZH/help/meta.md)

[使用 Relation](doc/ZH/help/relation.md)

[内置执行器](doc/ZH/help/built-in.md)

[一些业务情景的解决方法](doc/ZH/help/use-case.md)

## 注意

本系统还处于早期阶段，尤其是文档还需要不断的完善，有不妥之处，还请多提建议。

