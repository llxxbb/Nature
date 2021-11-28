# Nature

[English](README_EN.md)|中文

## 一句话介绍 Nature

Nature 是一个开发支撑平台，它将数据定义从代码中剥离出来，从而实现了**数据与系统**的解耦。

## Nature 要解决的核心问题及其本质

- 数据孤岛：功能驱动的开发模式使得数据的主导地位被掩盖和剥夺，服务化场景下数据各自为政，数据不能统一定义和管理，导致数据重复定义、数据边界不清等问题产生。
- 系统难维护：**数据定义被代码框死**，使得数据调整变得非常困难，当耦合多个系统时问题尤其严重。
- 高阶非功能性代码难以复用：如数据一致性，数据不变性，数据溯源，幂等，环境异常重试，延迟执行等，这些功能一般涉及到多个数据的协同处理，数据的分散性和差异性使得这一工作非常难以统一。

以上问题的本质是**数据治理**问题。依据帕累托法则，以上问题基本上占据着80%的开发和维护成本。

## Nature 的解决之道

要想从根本上解决这个问题，最直接的方法就是将数据定义从代码中剥离出来，并进行集中管理和控制。然而数据是数字化的，剥离出来之后需要有个独立的载体进行管理，并且这个载体能够与业务系统一起协作，以提供它们缺失的数据能力。这是一个业界的难点，Nature 的诞生便是为了解决这个难点。

在 Nature 平台上我们可以定义业务数据，以对整个业务体系**提供结构性支撑**；另外 Nature 还对业务数据间的流向关系进行定义，以对个业务体系**提供功能性支撑**。数据以及数据间的关系是整个业务体系的核心，这样 Nature 便解决数据孤岛和系统变更困难的问题。另外 Nature 对数据提供了统一的存储方式，为非功能性代码的通用化提供了保障，当然也为功能性复用提供了可能。

具体来讲 ，Nature 中的数据定义称之为 `Meta`，所有的 `Meta` 将构成企业的整个业务布局，而每个 `Meta` 可以定义自己在这个业布局中的位置；`Meta` 是静态的，与之对应的是运行时的数据实例 `Instance`，即 `Meta` 用于表示哪类数据而 `Instance` 用于表示哪个数据。Nature 中的数据间的关系称之为 `Relation`，用于表达两个 `Meta` 的上下游关系；在 `Relation` 上可以指定 `Executor`，用于实现上游 `Instance` 到下游 `Instance` 的转换，这个 `Executor` 可以是内置的也可以是外置的。

## 对现有开发模式的影响

因为数据即数据间关系在 Nature 里定义，现有的业务系统将被肢解成多个 `Executor` 并失去业务编排能力，同时大部分的非功能性代码也就有 Nature 来承担。

## Nature 名字的内含

大自然这个客观世界由事物及事物间的相互作用（关系）构成，本系统也是基于这两点进行构建，因此取名 Nature。另一方面本系统遵从生物进行的自然选择法则，无为而治，用选择来代替控制，以拟合生态系统中最原始，最朴素，最强大的事物运行规律。它体现在：

- 在高度分散化、大规模协作开发的时代，参与方之间是地位平等的，相互间的选择要比控制要高效的多。
- 从复杂度上来讲，控制需要回馈，而选择不需要回馈，所以要比控制简单。当系统规模不断扩大时复杂度的差异将更为突出。

同时也希望本系统像自然选择的生态法则一样，在历史舞台中是一个有意义的进化，一次成功的自然选择，一次为您的选择。

## 关键字

数据治理，服务治理，流程编排，低代码平台，数据孤岛、系统变更

## Nature 能力与特性

- **高阶复用**：Nature 可以直接复用某些业务能力。举例，通知能力可以通过配置的方式就可以对接审核、提醒，推广等不同的业务领域。Nature 之所以能够具有这样的能力是因为 Nature 对数据进行了标准化。
- **快速业务迭代**：Nature 将业务核心从业务系统解耦，这为业务调整提供了灵活性，同时 Nature 定义的每个 `Relation`就是一个**短流程**，这进一步提高业务的服用行。
- **数据不可变性及可溯源**：Nature 只增加数据，不改变数据，使数据保持不一致，并提供数据溯源能力。
- **内置的非功能性能力**：环境异常自动重试，逻辑异常转储，延时、定时处理，幂等，数据最终一致性
- **插件**：Nature 是一个平台，支持灵活的能力扩展，您可以自由的扩展选择器，执行器，前、后置处理等。
- **批处理**：Nature 提供了一种对海量数据处理过程状态管理的机制，使得处理可以分批进行。
- **上下文**：分用户上下文和系统上下文，通过上下文可以将一些特殊的数据在流程中传递。

有关技术特性的详细介绍请参考：[Nature 的技术特性](doc/ZH/help/characteristics.md)

## 如何使用

我们需要做下面的工作

**启动 Nature**：

1. 创建一个[mysql](https://www.mysql.com/) 或 [mariadb](https://mariadb.org/) 或 [Tidb](https://pingcap.com/en/) 数据库，并执行 [schema.sql](shell/schema.sql)
2. 配置好`.env` 文件中的`DATABASE_URL`属性以指向您创建的数据库
3. 启动 natrue.exe，retry.exe 和 manager.exe。

**业务开发**：

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

**有关JavaScript**:

js 在处理 i64 或 u64 时会有精度问题，为此 nature.exe 和 manager.exe 提供了相应的 JS 结尾的接口，这些接口使用 String 来代替 u64 或者 i64。

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

[Nature接口定义](doc/ZH/help/nature-interface.md)

[现有开发模式的问题分析及解决方法](doc/ZH/natureBusinessValue.md)。

如果您想了解下 Nature 的自然观，时空观，数学意义和哲学意义请阅读：[Nature 架构思想](doc/ZH/help/architecture.md)

如果您想在实际情况中了解如何应用 Nature 请阅读：[示例及功能讲解](nature-demo/README.md)，[一些业务情景的解决方法](doc/ZH/help/use-case.md)

如果您想了解 Nature 的技术特性以及这些特性是如何实现的请阅读：[Nature 的技术特性](doc/ZH/help/characteristics.md)

如果您想了解 Nature 与流式计算，消息系统，工作流等的区别请阅读：[与其他框架的比较](doc/ZH/compare.md)

如果您想使用好 Nature 请阅读：[使用 Meta](doc/ZH/help/meta.md)，[使用 Relation](doc/ZH/help/relation.md)，[内置执行器](doc/ZH/help/built-in.md)

## 其它说明

本项目的主体功能已经完善，现正在寻求合作，有意者请发邮件309577603@qq.com，微信： llxxbb76

[更新日志](doc/release/release.md)

## 路线图

### 核心功能

#### meta

- 私有状态：用于减少领域对象的数量。如订单和订单状态在业务上可以用一个领域对象来管理。其形式为 [meta_name]_S，可以定义初始状态。与私有状态想对应的是独立状态meta， 如 [负责人] meta , [负责人]是可以更换的所以[负责人]是一个状态数据。
- meta 模板: meta 类型为 T(emplate)，用于提供复用的 meta 信息。 模板类型的 Meta 不能生成Instance，模板可将 states、fields、 config 的内容作为模板使用。其它 meta 可使用 `from` 属性来关联模板来使用模板中的内容，也可以关联非模板meta，这是一种规范而不是强制措施，模板使用者可覆盖或追加内容。
- 伴生 meta：用于表示具有双向关系的实例，如用户属于某个组，组内有某个用户。伴生对象用于方便数据检索。伴生对象由 Nature 自动维护，不需要额外建立 relation。 其形式为 [meta_name]_R, 其中R为 Reverse 的首字母。条件：para 有且只有两个参数。
- 写入之前支持自定义的验证逻辑，适用于从外部输入数据到 Nature。
- 支持属性验证，更新时验证是否有`Relation`在使用，如有GUI则发出警告（待确认）
- 密文存储：简单方案，不区分用户，对称加密，公私钥方案见云版
- 可定义 instance 的生命周期，便于数据迁移自动化。

#### ralation

- relation 模板：模板只提供 settings 中除 is_template 和 from 之外的信息 。请设置 from 属性关联到一个已经存在的 `relation` 上。
- 动态路由，开关：出于安全上的考虑，在处理过程中，动态路由不能回归到静态路由。

#### 选择器

可方便支持外部扩展。

#### 内置convert

随机生成器：可指定生成的位数，形式可选 数字、字母+数字、字母

#### 执行器

- 可以通过 context 来 申请ID
- executor setting 中可复制，可增加，可移除context 中的内容

#### 数据库

对 elasticSearch， mongodb 进行支持。

#### ID 生成算法

目前是基于 hash 算法的，下一步考虑使用分布式ID算法。基于：snowflake

### 重试

执行重试任务时，检测是否忙碌，如果忙碌，返回忙碌信息。

### 示例

为了说明 Nature 可以很好的避免编码，我们找了个最常用的业务情景，来挖掘一下 Nature 非编码方式实现复杂业务逻辑的潜力，这个场景就是审核。

我们将模拟下面的审核场景：

- 业务类型：请假、报销、借款
- 审核方式：多级审核、多人审核、代理审核，指定领导审核。

### GUI

- 支持 meta 和 relation 的创建
- 更新配置时能够更新多个实例上的缓存
- 对 meta template 进行标记和支持，如可以对 meta template 进行过滤显式。
- meta 复制功能。
- meta 变更时提示是否生成新版本的 meta
- 伴生 meta 不会以独立的 Meta 在GUI中显示，但其所依附的主对象会进行标记
- 私有状态 meta 不独立展示，但其所依附的主对象会进行标记，创建关系时需要选择从主数据创建还是从私有状态数据创建。
- Relation 复制功能。
