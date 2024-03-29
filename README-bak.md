# Nature

[English](README_EN.md)|中文

## 一句话介绍 Nature

Nature 是一个开发支撑平台，适用于大型业务系统的开发，它将数据定义从代码中剥离出来，从而实现了**数据与系统的解耦**。

## Nature 要解决的核心问题

- 数据孤岛：功能驱动的开发模式使得数据的主导地位被掩盖和剥夺，服务化场景下数据各自为政，数据不能统一定义和管理，导致数据重复定义、数据边界不清等问题产生。
- 系统难维护：**数据定义被代码框死**，使得数据调整变得非常困难，当耦合多个系统时问题尤其严重。
- 高阶非功能性代码难以复用：如数据一致性，数据不变性，数据溯源，幂等，环境异常重试，延迟执行等，这些功能一般涉及到多个数据的协同处理，数据的分散性和多样性使得这一工作非常难以统一。

## Nature 的解决之道

### 数据定义的方式

传统方式下我们找到一个数据定义：我们首先需要找到系统，然后查看系统的接口文档，接口文档中的参数说明里会有数据的定义。即**通过功能来找数据**，这是一种低效的管理方式。因为**数据是系统间协作的桥梁**，数据是被多个系统所**共享**的，所以将数据定义**局限**到一个接口参数的定义中是不合适的。一方面开发小组的思想会有局限性，另一方面一旦以这种形式**固化下来**，后期改动会非常困难，这是**高维护成本的根源所在**。

既然是共享的，那么就应该把数据定义独立于各个系统之外，并集中放在一个地，使之**显式可见**，以方便大家查阅。另外还需要**将数据定义作为顶层组织方式**，而不是将系统作为顶层组织方式。这也符合人的思维方式：**目标导向**。数据是目的，而系统只是某种实现方式。这会有助于让我们**过滤掉大量的干扰信息**而有效聚焦于目标管理上。

由此 Nature 诞生了，数据的定义可以被集中管理了，那么 Nature 该如何和这些系统一起协助呢？只是当一个数据定义的验证中心吗？这显然是不够的。当下是数据时代，管理的难点之一在于**数据之间错综复杂的关系管理**。

### 数据间的关系

在大型系统中，出于数据量、并发能力等因素的考虑，我们一般会对数据库进行垂直和水平拆分，同时我们已经在刻意回避 join 的使用，这种拆分使我们失去了全局视角，进而失去对全局数据的管理能力。也就是说我们与**关系型数据库**的理念渐行渐远！

然而数据间的关系对业务来讲至关重要！因为**这层关系的可见性缺失**，直接导致团队协作成本和系统维护成本高企不下。所以我们有必要以某种方式重新建立起这些数据的关系，以指导和约束系统的开发。所以 Nature 做了第二件事，对数据之间的关系进行管理。

### 数据的一致性

数据一致性是系统在运行期间**维持数据关系正确性**的一种保证。我们一般寄希望于数据库的事务来保证，但业务系统的分布式特性，使得我们必须自己来面对这种情况。这是一种技术门槛比较高的工作，很难有一种拿来即用的方案来应对，且可维护性比较差。

借助于对数据及其之间的关系的**抽象**，Nature 可以以一种统一的方式来**一劳永逸**处理数据的一致性问题，这要求 Nature 对数据处理有完全的**控制权** 。

### 数据处理引擎

当 Nature 接管数据处理的控制权后，业务系统之间必须经过 Nature 才能相互通讯，这样系统之间将不存在耦合了，系统的职责将更为单一，技术门槛更低，功能迭代和维护将更容易。

Nature 只关注于数据，从数据的定义，数据之间的关系，以及保证数据关系的正确表达，即 Nature 是一个数据处理引擎，它**颠覆了技术主导数据的传统模式**，反其道而行之，用数据来规范、制约系统的行为。

## Nature 的工作方式

Nature 中的数据定义称之为 `Meta`，代表业务中的**某一类数据**，如订单类数据。每个 `Meta` 都有自己的唯一标识。所有的 `Meta` 以**业务领域**的形式组成一棵具有多个层级的单根树，从而构成企业的整个业务布局，为管理者提供可以统观全局的**静态视角**。

与 `Meta` 对应的是在运行时的产生的数据 `Instance`，代表业务中的**某一笔数据**，如某条订单数据。每个 `Instance`只从属于一个 `Meta`。

Nature 中的数据间的关系称之为 `Relation`，用于表达两个 `Meta` 的**上下游关系**，`Relation`为管理者提供了统观全局的**动态视角**；在 `Relation` 上可以指定 `Executor`，用于实现上游 `Instance` 到下游 `Instance` 的转换。如，订单和发票之间可以有一个`Relation`。至于如何依据一笔订单生成一条发票数据，那是 `Executor` 的工作，`Executor`由 Nature 来合适的时机进行调度。

## Nature 名字的内含

大自然这个客观世界由事物及事物间的相互作用（关系）构成，本系统也是基于这两点进行构建，因此取名 Nature。另一方面本系统遵从自然选择法则，无为而治，用选择来代替控制，以拟合生态系统中最原始，最朴素，最强大的事物运行规律。它体现在：

- 在高度分散化、大规模协作开发的时代，参与方之间是地位平等的，相互间的选择要比控制要高效的多。
- 从复杂度上来讲，控制需要回馈，而选择不需要回馈，所以要比控制简单。当系统规模不断扩大时复杂度的差异将更为突出。

## 关键字

数据治理，服务治理，流程编排，低代码平台，数据孤岛、需求变更

## Nature 能力与特性

- **高阶复用**：因为数据定义独立于代码而存在，这使我们可以更好的设计和调整业务数据，使其更容易实现通用性。同时每个 `Relation`都是一个最小可复用单元，在增强灵活性的同时，也进一步提高了业务的可复用性。比如，我们可以非常容易的使通知能力来承接审核、提醒，推广等不同的业务。
- **快速业务迭代**：一方面数据定义的集中统一，会大幅度减少沟通成本；另一方面，Nature 打破了系统间的耦合，使迭代没有那么多的牵绊。这两点都会使迭代提效加速。
- **数据不可变性及溯源**：Nature 只增加数据，不改变数据，使数据保持不一致，并提供数据溯源能力。
- **内置的非功能性能力**：环境异常自动重试，逻辑异常转储，延时、定时处理，幂等，数据最终一致性等
- **插件**：Nature 是一个平台，支持灵活的扩展，您可以自由的扩展选择器，执行器，前、后置处理等。
- **批处理**：Nature 提供了一种批量处理海量数据的机制。
- **上下文**：分用户上下文和系统上下文，通过上下文可以将一些特殊的数据在流程中传递。

有关技术特性的详细介绍请参考：[Nature 的技术特性](doc/ZH/help/characteristics.md)

## 快速开始

我们需要做下面的工作

**启动 Nature**：

1. 创建一个[mysql](https://www.mysql.com/) 或 [mariadb](https://mariadb.org/) 或 [Tidb](https://pingcap.com/en/) 数据库，并执行 [schema.sql](shell/schema.sql)
2. 配置好`.env` 文件中的`DATABASE_URL`属性以指向您创建的数据库
3. 启动 natrue.exe，retry.exe 和 manager.exe。

**基于 Nature 开发**：

1. 在 [Meta](doc/ZH/help/meta.md) 数据表里定义多个业务对象，如：我们定义`订单`和`订单账`两个业务对象
   
   ```sql
   INSERT INTO meta (full_key, description, version, states, fields, config) VALUES
   ('B', 'sale/order', 'order', 1, '', '', ''),
   ('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}'); 
   ```

2. 在 [relation](doc/ZH/help/relation.md) 数据表使定义关系将多个业务对象关联起来，并在 `Relation` 里设置 `Executor` 用于业务对象间的转换（相当于流式计算中的 map），如上面的`订单`和`订单账`可以有这样的定义：
   
   ```sql
   INSERT INTO relation
   (from_meta, to_meta, settings)
   VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"states":{"add":["unpaid"]}}}');
   ```

3. 写代码实现您上面定义的 [Executor](doc/ZH/help/executor.md)。如果是基于 Http 的请在完成后启动它，如果是基于类库的请将之放到 与 nature.exe 相同的目录下。如对于`订单`和`订单账`来讲这个逻辑是：
   
   - 生成一个`订单账`业务对象
   - 从传入的`订单`中提取所有商品的应收款项记为该`订单账`的应收
   - 将`订单账`对象返回给 Nature 以驱动下一环节的处理

**进行业务处理**：

对 Nature 发起 http post 请求，如将`订单`数据提交数据到 Nature，Nature 会自动按顺序驱动 `Executor` 来完成所定义的任务。

## Natrue 的详细资料

- [使用 Meta](doc/ZH/help/meta.md)

- [使用 Relation](doc/ZH/help/relation.md)

- [内置执行器](doc/ZH/help/built-in.md)

- [任务](doc/ZH/help/task.md)

- [Nature接口定义](doc/ZH/help/nature-interface.md)

**有关JavaScript**:

js 在处理 i64 或 u64 时会有精度问题，为此 `nature.exe` 和 `manager.exe` 提供了相应的 JS 结尾的接口，这些接口使用 String 来代替 u64 或者 i64。

## 图形化管理界面

[图形化管理界面](https://github.com/llxxbb/Nature-Manager-UI)可实现三种模式下的管理：

### 关系模式

该模式下，你可以设计如何让业务运转起来。

![relation.png (1769×1036) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/relation.png?raw=true)

### 领域模式

该模式下，可方便进行业务领域的划分。

![domain.png (933×428) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/domain.png?raw=true)

### 数据流模式

该模式下，你可以方便地看到数据之间是如何流转的。

![instance.png (1507×1068) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/instance.png?raw=true)

## 深入了解Nature

[现有开发模式的问题分析及解决方法](doc/ZH/natureBusinessValue.md)。

如果您想了解下 Nature 的自然观，时空观，数学意义和哲学意义请阅读：[Nature 架构思想](doc/ZH/help/architecture.md)

如果您想在实际情况中了解如何应用 Nature 请阅读：[示例及功能讲解](nature-demo/README.md)，[一些业务情景的解决方法](doc/ZH/help/use-case.md)

如果您想了解 Nature 的技术特性以及这些特性是如何实现的请阅读：[Nature 的技术特性](doc/ZH/help/characteristics.md)

如果您想了解 Nature 与流式计算，消息系统，工作流等的区别请阅读：[与其他框架的比较](doc/ZH/compare.md)

## 其它说明

本项目的主体功能已经完善，现正在寻求合作，有意者请发邮件309577603@qq.com，微信： llxxbb76

[更新日志](doc/release/release.md) 
