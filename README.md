# Nature

[English](README_EN.md)|中文

## 一句话介绍 Nature

Nature 是一个开发支撑平台，它将数据定义从代码中剥离出来，并以配置化的方式进行集中、统一的定义和管理，从而实现了**数据与业务系统**的解耦。

## 关键字

数据治理，服务治理，流程编排，低代码平台，数据孤岛、系统变更

## Nature 要解决的核心问题及其本质

- 数据孤岛：服务化场景下数据各自为政，会存在业务数据重复和耦合的问题。从战略上不利于公司进行统一规划和管理。
- 系统变更困难：**数据定义被局部代码框死**，跨服务的数据调整变得非常困难。
- 非功能性代码难以复用：如数据一致性，数据不变性，数据溯源，幂等，环境异常重试，延迟执行等。依据帕累托法则，这些代码是非核心的，但却占据着80%的开发和维护成本。

我们认为以上问题的**本质是数据管理问题**。当数据的控制权掌握在不同研发团队手中时，自然对系统协作造成不利影响，就会导致数据孤岛问题，同时使得基于存储的非功能性代码难以复用。

## Nature 的解决之道

要想从根本上解决这个问题，最直接的方法就是将数据定义从代码中剥离出来，并进行集中管理和控制。然而数据是数字化的，剥离出来之后需要有个独立的载体进行管理，并且这个载体能够与业务系统一起协作，以提供它们缺失的数据能力。这是一个业界的难点，Nature 的诞生便是为了解决这个难点。

在 Nature 平台上我们可以定义业务数据，以对整个业务体系**提供结构性支撑**；另外 Nature 还对业务数据间的流向关系进行定义，以对个业务体系**提供功能性支撑**。数据以及数据间的关系是整个业务体系的核心，这样 Nature 便解决数据孤岛和系统变更困难的问题。另外 Nature 对数据提供了统一的存储方式，为非功能性代码的通用化提供了保障，当然也为功能性复用提供了可能。

具体来讲 ，Nature 中的数据定义称之为 `Meta`，所有的 `Meta` 将构成企业的整个业务布局，而每个 `Meta` 可以定义自己在这个业布局中的位置；`Meta` 是静态的，与之对应的是运行时的数据实例 `Instance`，即 `Meta` 用于表示哪类数据而 `Instance` 用于表示哪个数据。Nature 中的数据间的关系称之为 `Relation`，用于表达两个 `Meta` 的上下游关系；在 `Relation` 上可以指定 `Executor`，用于实现上游 `Instance` 到下游 `Instance` 的转换，这个 `Executor` 可以是内置的也可以是外置的。

## 对现有开发模式的影响

因为数据即数据间关系在 Nature 里定义，现有的业务系统将被肢解成多个 `Executor` 并失去业务编排能力，同时大部分的非功能性代码也就有 Nature 来承担。

## 关键字

数据治理，服务治理，流程编排，低代码平台，数据孤岛、系统变更

## Nature 能力与特性

- **高阶复用**：Nature 可以直接复用某些业务能力。举例，通知能力可以通过配置的方式就可以对接审核、提醒，推广等不同的业务领域。Nature 之所以能够具有这样的能力是因为 Nature 对数据进行了标准化。
- **快速业务迭代**：Nature 能够帮助企业提炼出业务核心并进行独立管理，这使得企业非常容易进行业务调整。

- **数据不可变性**：Nature 只增加数据，不改变数据，因为改变就有可能造成数据的不一致。对于状态数据 Nature 为变更提供版本能力来支持数据的不可变性，在 Nature 内部版本信息是自动维护的。
- **数据处理过程可追溯**：每个数据实例都会记录来源，不会遗失任何处理环节。

- **短流程**：一个 `Relation` 就是一个流程，且是一对一的，Nature 的选择而非控制理念造就这种轻量模型，这为流程拼接提供了巨大的灵活性。 
- **内置的非功能性能力**：环境异常自动重试，逻辑异常转储，延时、定时处理，幂等，数据最终一致性
- **插件**：Nature 是一个平台，支持灵活的能力扩展。
- **选择器**：提供用于触发数据转换的条件。Nature 提供了灵活的可扩展的选择器设置。
- **执行器**：Nature 提供了可扩展的基于不同协议的执行器。目前支持 rust 本地包和 http 两种执行器。
- **前置处理**：用于数据转换前的处理，如数据格式调整。Nature 提供了一些内置的前置处理器，您也可以定义自己的前置处理器。这个一般是技术性处理，而不需要引入一个独立的业务对象：`Meta`。
- **后置处理**：同前置处理器，只不过是应用于转换后的数据处理。
- **批处理**：Nature 提供了一种对海量数据处理过程状态管理的机制，使得处理可以分批进行。
- **上下文**：分用户上下文和系统上下文，通过上下文可以将一些特殊的数据在流程中传递。

有关技术特性的详细介绍请参考：[Nature 的技术特性](doc/ZH/help/characteristics.md)


## 如何使用

我们需要做下面的工作

**启动 Nature**：

1. 创建一个[mysql](https://www.mysql.com/) 或 [mariadb](https://mariadb.org/) 或 [Tidb](https://pingcap.com/en/) 数据库，并执行 [schema.sql](shell/schema.sql)
2. 配置好`.env` 文件中的`DATABASE_URL`属性以指向您创建的数据库
3. 启动 natrue.exe和retry.exe。

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

## 路线图

### 核心功能

#### meta

- 添加字段 name :  友好名称。
- meta 模板: meta 类型为 T(emplate)，用于提供复用的 meta 信息。 模板类型的 Meta 不能生成Instance，模板可将 states、fields、 config 的内容作为模板使用。其它 meta 可使用 `from` 属性来关联模板来使用模板中的内容，也可以关联非模板meta，这是一种规范而不是强制措施，模板使用者可覆盖或追加内容。

- 伴生 meta：用于表示具有双向关系的实例，如用户属于某个组，组内有某个用户。伴生对象用于方便数据检索。伴生对象由 Nature 自动维护，不需要额外建立 relation。 其形式为 [meta_name]_R, 其中R为 Reverse 的首字母。条件：para 有且只有两个参数。

- 可定义 instance 的生命周期，便于数据迁移自动化。

- 私有状态：用于减少领域对象的数量。如订单和订单状态在业务上可以用一个领域对象来管理。其形式为 [meta_name]_S，可以定义初始状态。与私有状态想对应的是独立状态meta， 如 [负责人] meta , [负责人]是可以更换的所以[负责人]是一个状态数据。

#### ralation

- 添加字段：description，对关系的作用进行描述。
- relation 模板：模板只提供 settings 中除 is_template 和 from 之外的信息 。请设置 from 属性关联到一个已经存在的 `relation` 上。

#### 选择器

可方便支持外部扩展。

#### 内置convert

随机生成器：可指定生成的位数，形式可选 数字、字母+数字、字母

#### 数据库

对 elasticSearch， mongodb 进行支持。

### 示例

为了说明 Nature 可以很好的避免编码，我们找了个最常用的业务情景，来挖掘一下 Nature 非编码方式实现复杂业务逻辑的潜力，这个场景就是审核。

我们将模拟下面的审核场景：

- 业务类型：请假、报销、借款

- 审核方式：多级审核、多人审核、代理审核，指定领导审核。

### GUI

- 支持 meta 和 relation 的创建
- 对 meta template 进行标记和支持，如可以对 meta template 进行过滤显式。
- meta 复制功能。
- meta 变更时提示是否生成新版本的 meta
- 伴生 meta 不会以独立的 Meta 在GUI中显示，但其所依附的主对象会进行标记
- 私有状态 meta 不独立展示，但其所依附的主对象会进行标记，创建关系时需要选择从主数据创建还是从私有状态数据创建。
- Relation 复制功能。
