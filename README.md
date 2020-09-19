# Nature

[English](README_EN.md)|中文

## Nature 是什么？

### 极简开发平台 && 流式计算引擎

Nature 是一个**基于网络**的适用于各种编程语音的**极简开发平台**。Nature 提供了**面向业务**的**基于配置**的**流式计算引擎**，业务的差异性不再体现为传统的一个个独立的代码系统，开发者不用考虑业务流程控制、时间调度、幂等、失败重试和高并发等问题。所有的开发工作都被分割成**互不影响**的**可独立编程的最小单元**，每个单元只需关注一个输入的业务对象和一个输出的业务对象即可，而这些业务对象的流转控制和存储完全由 Nature 来代劳。Nature **使业务对代码具有完全的控制能力**，不会受既有系统这样或那样的约束。这会有两个方面的重要意义：一是更容易塑造你的系统，二是可以大幅度减少开发投入。

### 数据中心

Nature 提供了**统一的数据存储模型**。开发者不再需要为每个业务实体设计数据表，Nature 使用统一的类似于`Key-Value`机制存储所有的数据实例。相对于传统项目有下面几点您可能比较关注：

- 数据检索：可以利于 Nature 的流式计算机制加工出您想检索的任何数据来。
- 数据库的容量：可以考考使用 [Tidb](https://pingcap.com/en/) 来作为 Nature 的后端存储引擎。

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

5. 配置好.evn以指向您创建的数据库，启动 natrue.exe 和 retry.exe

6. 对 Nature 发起 http 请求，以POST方式将`订单`数据提交数据到 Nature，请参考[提交数据到 Nature](doc/ZH/help/submit-to-nature.md)

## 深入了解Nature

[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)

[Nature 架构说明](doc/ZH/help/architecture.md)

[使用 Meta](doc/ZH/help/meta.md)

[使用 Relation](doc/ZH/help/relation.md)

[内置执行器](doc/ZH/help/built-in.md)

[一些业务情景的解决方法](doc/ZH/help/use-case.md)

## 注意

本系统还处于早期阶段，有不妥之处，还请多提建议。

