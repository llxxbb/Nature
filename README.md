# Nature

[English](README_EN.md)|中文

## Nature 是什么？

以领域驱动为依据，集数据定义、处理、存储、查询为一体的业务系统辅助开发平台。

## Nature口号？

让数据主导技术团队而不是反过来。

## 现有业务系统的问题

系统复杂的一个重要原因在于**技术团队主导数据**，这会导致**数据与技术的耦合**！最直接的证据就是接口，接口是一种技术实现，但作为数据的入参和出参必须依附于接口才有意义。另一个证据就是，很多时候只有深入代码我们才能了解数据之间的关系变化。

技术是复杂多变的，当技术这一手段或工具”绑架“数据后，数据便变成了技术的奴仆，使业务的管理和调整变得困难且成本高昂。具体体现在：

- 技术团队对业务的理解是**间接的**，有延后性和偏差性问题。

- 技术团队会**泛化**业务数据的范围和边界，如技术数据、临时数据、中间数据等各种各样的数据和业务数据糅合在一起。

- 随着业务的**迭代**，技术团队会让数据变得复杂难懂且使系统难以维护。

## Nature 要解决的核心问题

- **全局视角**：杜绝“近视”，避免陷入功能的泥潭而迷失目标，能够提供简明的全局数据视图，为业务决策提供更好的支持，利于决策的传达并提升全员的参与度，可以从根本上解决服务治理难题。
- **数据共享**：数据是企业的重要诉求，数据是不同部门间协作的桥梁，避免系统间的杂乱的调用关系，不用为边界问题而吵架。所以数据应该是共享的而不是技术团队的的私有物品。非技术团队也应该有管理数据的能力。
- **过程可见**：数据不会被“绑架”、僵化，使数据流转的所有环节可见且易于调控。而不是去面对晦涩难懂的接口调用。且对技术团队进行强制约束。
- **调整便捷**：能够及时准确的对业务进行调整，杜绝已有技术实现对新需求产生羁绊作用。
- **统一标准**：统一和标准化数据的定义和处理方式，以此提供更高阶的复用性，去除多样性所带来的高维护成本。

## Nature 的解决之道

Nature 的实现类似于传统的数据库，也强调数据定义，以及数据间的关系。区别在于 Nature 以领域的组织数据定义，以数据关系来约束业务流程开发，并辅以各种增值能力来简化流程开发。

### 数据定义

Nature的数据定义（称之为 `Meta`）可看作是**大众化的目标定义**。之所以称之为目标定义，是因为系统的产物就是数据，数据就是我们的目标，朴素、本质，直观、简洁且可视化，所以 Nature 的数据定义定义了系统的目标；之所以称之为大众化的，是因为不需要很高的技术门槛。不同于数据库的数据定义：

- **纯粹的业务语义**：这即简化了使用，又避免了技术数据、临时数据等对业务数据的干扰，提升了数据质量。

- **以领域驱动为指导原则**：将所有数据挂到领域树上，让数据有了从属和组织关系，既利于宏观调整（横向层次关系），又利于精细化管理（纵向从属关系）。业务管理便捷、及时、到位，为管理者提供统观全局的**静态视角**。

- **形式统一**：规避了不同数据库产品间的技术差异，大大降低技术门槛。使得非技术人员能够直接参与数据管理工作，减少了技术支出。

- **数据定义版本化**：新旧需求彼此不受影响，且能展示业务演变轨迹。

### 数据间的关系

Nature 间的数据关系（称之为 `Relation`）可看作是**大众化的行为准则**。我们既然有了大众化的目标，就相应的应该有实现这些目标的大众化的行为准则。

所谓的大众化的，是说 Nature 数据关系的内涵简单，如下：

- 一个关系只有两个相关方，一个为输入一个为输出，用于表达两个 数据定义的**上下游关系**，为管理者提供统观全局的**动态视角**。

- 当多方产生关系时，可分解成两两关系。

为什么叫行为准则呢？是说 Nature 的数据关系可对业务流程的开发进行强制约束。不同于数据库的关系，Nature 接管了所有的输入和输出，对系统的行为进行了强制控制。这种控制剥夺了传统系统对数据关系的隐式管理，替代了传统系统的分散的、晦涩的、定制的代码控制方式。其意义体现在：

- 业务流转描述更直接且准确。使技术团队能够更好的知道业务需要什么。

- 数据关系管理透明、简洁、集中。

- 使接口形式变得单一且统一，开发变得更简单。

- 数据处理方式的统一，可以让 Nature 进一步赋能，这进一步减轻了技术团队的负担。

- 短流程：只有两两关系，拼接灵活，可提高流程复用度

### 执行器和数据实体

执行器（称之为 `Executor`）便是上面统一接口的具体体现。数据关系强调的是数据间存在因果关系，至于因果如何转化则需要借助于执行器来实现。Nature 为执行器提供给定的输入，执行器则必须产出 Nature 要求的输出。  

如，订单和发票之间可以有一个 `Relation`。至于如何依据一笔订单生成一条发票数据，那便是 `Executor` 的工作，`Executor`由 Nature 在合适的时机进行调度。

上面的输入和输出都是数据实体（称之为 `Instance`）。数据定义用于表示某一类数据，而数据实体则表示该类数据下的某一个具体数据。每个数据实体只能属于一个数据定义。 

## Nature 名字的内含

大自然这个客观世界由事物（数据）及事物间的相互作用（数据关系）构成，本系统也是基于这两点进行构建，因此取名 Nature。

自然的另外一层意思是**自然选择**，自然选择是生态系统的基本法则，法则没有去控制，无为而治，但却显现出了强大的无形控制。对于一个系统来讲，规模越大，控制的难度也成几何级增加，与其控制繁杂的具体事物，不如守住简单统一的规则。 

Nature 的自然选择体现在数据关系的处理上：下游选择上游而不是上游控制下游。上游没有必要知道有多少个下游，也不用关心如何流转到下游，而下游只管提出哪个上游能满足我即可。不选择控制的原因在于：

- 在高度分散化且大规模协作开发的时代，参与方之间是**地位平等**的，相互间的选择要比控制要高效的多。
- 从复杂度上来讲，控制需要回馈，而选择不需要回馈，所以先择要比控制简单。当控制的层级及链路随着系统的规模不断扩大时复杂度将更为突出。

Nature 遵从第一性原理：选择遵从内心的需要，不需要控制；这里的控制是对业务的控制而不是对技术的控制。

Nature 用一对一这种简单关系来诠释完整的业务模型，其复杂性是自然而然“涌现”出来的，不是我们刻意设计出来的；“涌现”一词借鉴于《失控》。

## Nature 价值

- 决策的标准化：数据定义就是决策定义，Nature 用准确简洁的定义来描述业务语义并可强制约束系统的行为，大幅度减少人员沟通、歧义数据转换成本。

- 执行标准化：Nature 用选择的方式简化并统一了接口，去除了系统间的耦合及边界问题，每个接口都准确来源于业务的需要，简单、直接、易于维护。

- 数据规范和纯化：纯粹的业务数据，统一的定义和处理方式，为行业数据标准化，及流程复用提供了基础，为更广层次的业务协作提供支撑平台。

基于上面的价值，Nature 还可以将价值进一步延申，如下：

- **提供业务洞察力**：全局的动态、静态视角为业务迭代提供了洞察力。
- **原子业务**：业务被拆分成不可拆分的最小管理单元，实现对业务的深度管理的同时也保留了很强的灵活性，为业务的调整及业务复用提供便利。如审核、提醒、推送等不需要专业的设计能力便可非常容易的实现通用性，可方便的对接其他业务。
- **业务弹性**：版本化机制，为试错和遗留资产提供了弹性空间。
- **快速业务迭代**：业务流程不需要编码了！低门槛的业务准则使得业务调整非常便利，且不受已有技术实现的掣肘。
- **快速开发迭代**，主要体现：
  - Nature 打破了系统间的耦合，使开发更容易聚焦到业务实现而不是业务边界上。
  - Nature 接管了输入、输出，减少了开发的职责范围。
  - Nature 在执行器上进行了统一的赋能，如数据版本化，幂等、环境异常重试、延迟执行、数据不变性、数据一致性等，这大大降低了对开发的技术要求。

## 业务特性

- 数据定义
  
  - 版本化：为新旧业务的过度提供支持。
  
  - 自组织、自描述：Nature 为业务数据提供领域管理能力，为复杂的业务模型提供便利。

- 状态数据：如订单的待支付、待发货等状态。

- 第三方数据：可以将第三方数据用于 Nature 数据的唯一标记（称之为参数化），以方便与现有业务系统继承。

- 定时执行：执行器可以在某一特定时间执行。应用场景，如基于分钟、小时、天等的汇总。

- 延迟执行：执行器会在输入就绪后一定时间间隔后开始执行。

## 技术特性

- 高性能：
  
  - 核心用 rust 构建
  
  - 高并发：采用异步编程模型

- 可水平扩展：Nature 本身可水平扩展部署，数据库层可使用 tidb 进行水平扩展。

- 自动重试：如果因环境因素导致执行器执行失败，Nature 会自行重试。

- **创造历史**：也可称之为**数据不变性**，数据一旦提交或生成，便不可改变。状态数据如果发生变化，则版本会随之增加，这为溯源问题排查提供了便捷。即使相同数据多次提交，也不会改变结果，此特性也称之为**幂等**。

- **任务跟踪**：Nature 对每条数据的每一步处理都会进行独立跟踪，以保证业务最终完成，此特性于幂等一起便可保证**数据最终一致性**。

- **插件**：Nature 是一个平台，支持灵活的扩展，您可以自由的扩展选择器，执行器，前、后置处理等。

- **批处理**：Nature 提供了一种批量提交、处理海量数据的机制。

- **上下文**：分用户上下文和系统上下文，通过上下文可以将一些特殊的数据在流程中传递。 

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

- [执行器](doc/ZH/help/executor.md)

- [任务](doc/ZH/help/task.md)

- [Nature接口定义](doc/ZH/help/nature-interface.md)

**有关JavaScript**:

js 在处理 i64 或 u64 时会有精度问题，为此 `nature.exe` 和 `manager.exe` 提供了相应的 JS 结尾的接口，这些接口使用 String 来代替 u64 或者 i64。

## 深入了解Nature

[现有开发模式的问题分析及解决方法](doc/ZH/natureBusinessValue.md)。

如果您想了解下 Nature 的自然观，时空观，数学意义和哲学意义请阅读：[Nature 架构思想](doc/ZH/help/architecture.md)

如果您想在实际情况中了解如何应用 Nature 请阅读：[示例及功能讲解](nature-demo/README.md)，[一些业务情景的解决方法](doc/ZH/help/use-case.md)

如果您想了解 Nature 的技术特性以及这些特性是如何实现的请阅读：[Nature 的技术特性](doc/ZH/help/characteristics.md)

如果您想了解 Nature 与流式计算，消息系统，工作流等的区别请阅读：[与其他框架的比较](doc/ZH/compare.md)

## 其它说明

[图形化管理界面](https://github.com/llxxbb/Nature-Manager-UI)

[杂谈](doc/miscellaneous.md)

请参考：[服务治理咋这么难？我想得换个治法了](https://www.cnblogs.com/llxxbb/p/serviceGovernance.html)。

本项目的主体功能已经完善，现正在寻求合作，有意者请发邮件309577603@qq.com，微信： llxxbb76

[更新日志](doc/release/release.md) 
