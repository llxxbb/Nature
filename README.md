# Nature

此项目由[Select](https://github.com/llxxbb/Select)项目代替，不在维护，

[English](README_EN.md)|中文

## Nature 是什么？

Nature 可以让你在宏观层面**全面了解、把控和规划数据**，并以最直观、最简洁的方式将生产数据呈现给决策者。它能够对**数据流转**进行编排，这种编排是一种系统可理解的**强约束型的需求**，可直接作用于应用系统的开发，从而避免了传统需求的不确定性以及为此付出的沟通转化成本。注意这不是功能编排，Nature **用数据来连接世界**，避免你陷入功能的泥潭而迷失目标，因此你的管理将非常的直接、简洁和有效。

Nature 认为当前**服务治理的核心矛盾**在于服务单一职责与服务数量庞大且交互复杂之间的矛盾。虽然出现了中台的概念来使交互有了明确的分层分层，但这仅仅是业务的条理化，并不能在技术形式上带来本质简化，因为**交互繁杂**的本质问题是服务间的**直接控制**，控制强有力的方式是内聚而不是耦合，但服务间的调用恰恰体现的是耦合而不是内聚，因为服务之间是相互独立的个体，地位是平等的，于是就有了意愿是控制但形式是协作的矛盾。请参考：[服务治理咋这么难？我想得换个治法了](https://www.cnblogs.com/llxxbb/p/serviceGovernance.html)

Nature 另辟蹊径，摆脱服务的限制，以目标为导向；并对**多样化的功能**进行了抽象，将其抽象为转换器；因为系统工作的实质就是将一个个不同的小目标串起来实现我们最终的诉求，而转换器则行使“串起来”这一职责。这里的目标就是数据，而转换器就是功能；Nature 将数据和符号化的转换器进行了统一管理，借此 Nature 可以将核心流程控制从传统服务中剥离出来，并形成独立的控制中心：大脑。这可大幅度简化系统的整体复杂度。Nature 为此提供了一套新的简单、统一和规范的开发模式。有关 Nature 的架构思想、自然观、时空观、哲学观以及数学观请参见：[Nature 架构思想](doc/ZH/help/architecture.md)。

有了这个统一和规范的模式，我们就可以将一些原先需要定制化开发的功能进行标准化处理，而这些功能往往是业务系统中最占工作时间、最复杂，最难维护且最不显山露水的，如幂等、重试、高并发、数据最终一致性等。这会进一步简化您的服务并大幅度提升整个系统的健壮性和可维护性。请参考：[现有开发模式的问题分析及解决方法](doc/ZH/natureBusinessValue.md)。

## 理论基础

我们只所以建立系统就是为了处理数据。而数据的大规模并行处理 Google 给出了 MapReduce 方案。Nature 在此基础上进行了改进，其处理模式为： **数据 --map--> 数据 --map--> 数据...**它将 MapReduce 简化成了 Map, Nature 并不是丢弃了 reduce，在 Nature 看来 reduce 是多条数据对一条数据的映射，可以当做一个 Map 的特例。 这种简化的模式可以让我们更容易聚焦到数据以及数据之间的关系上，而不是如何处理这些数据。

因为 map 是一个动作，它含有转换的意思，它是功能化的是 Nature 对功能的抽象，Nature 用它来对运行过程中的数据流转进行控制，因此**数据 --map--> 数据是Nature 的运行时模式**。但从管理角度来看，我们更关注结果而不是过程。因此 Nature 进一步简化了这个模式，把 map 去掉了，新的模式变成了：**数据 --> 数据 --> 数据...**。这是 **Nature 的设计时模式**。该模式可以让我们更简洁的关注**数据间的内在联系**，这些联系是系统为之运转的基础。

也就是说数据和数据间的关系是整个系统的顶层设计，是对功能实现的指导和约束。在此基础上我们只要在关系上附加上一个 map 功能，系统便可以运转起来。这样任何复杂的流程都可以通过该简单机制串接起来来形式职能。Nature 用 `Meta` 来表示数据，用 `Relation` 来表示数据之间的关系，用`Executor`来表示 map，Nature 主要用这三个元素来控制整个系统的运转。

## 关键字

数据治理、服务治理，流程编排，低代码平台

## 技术特性

- **高阶复用**：不同于程序中方法、模块、类等的复用，这些复用必须通过编码才能对接，Nature 的复用无需编码。举例，通知能力可以通过配置的方式就可以对接审核、提醒，推广等不同的业务领域。Nature 之所以能够具有这样的能力是因为 Nature 对数据进行了标准化。

- **短流程**：既一个 `Relation` 就是一个流程，非常的轻量且为流程拼接提供了巨大的灵活性。

- **快速业务迭代**：Nature 为 `Meta` 提供了版本能力，基于此你可以在不影响既有流程的情况下，构建新的流程并方便切换，这使得您非常容易的进行业务调整。
- **数据不可变性**：Nature 只增加数据不改变数据，因为改变就有可能造成数据的不一致。对于状态数据 Nature 为变更提供版本能力来支持数据的不可变性，在 Nature 内部版本信息是自动维护的。
- **数据处理过程可追溯**：Nature 运行时自动会记录数据实例之间的关系，使所有数据可追溯。
- **自动重试**：Nature 对环境异常和逻辑异常进行了明确和规范，当遇到环境异常时，会自动重做任务。
- **错误数据转移**：Nature 不会丢掉任何数据，即使遇到逻辑错误，也能够保存下来让您进行审查，当故障排除后，可以将中断的任务重新导入到任务数据表，Nature 将会再次执行这些任务，直到成功完成。
- **延时、定时处理**：Nature 可以让你指定数据转换的时间，这对统计分析类数据会非常有意义。
- **幂等**：Nature 对所有参与数据转换协作的系统没有幂等的要求，但 Nature 会记住并使用他们第一次成功处理的结果，基于此 Nature 实现了自身的幂等性。
- **数据一致性**：Nature 支持数据的最终一致性。
- **选择器**：选择器提供用于触发数据转换的条件。Nature 提供了灵活的可扩展的选择器设置。
- **执行器**：Nature 提供了可扩展的基于不同协议的执行器。目前支持 rust 本地包和 http 两种执行器。
- **前置处理**：用于数据转换前的处理，如数据格式调整。Nature 提供了一些内置的前置处理器，您也可以定义自己的前置处理器。这个一般是技术性处理，而不需要引入一个独立的业务对象：`Meta`。
- **后置处理**：同前置处理器，只不过是应用于转换后的数据处理。
- **批处理**：Nature 提供了一种对海量数据处理过程中中间状态的管理机制，使得处理可以分批进行。

有关技术特性的详细介绍请参考：[Nature 的技术特性](doc/ZH/help/characteristics.md)

## 对现有开发模式的影响

主要体现在以下几个方面。

- **数据驱动**（目标驱动）：传统开发大多是面向功能的，功能一般通过接口的形式对外提供服务，一个系统往往存在着大量的接口，不但开发维护困难，而且很容易让人陷入功能旋涡而迷失掉目标，尤其是缺少全局视角下的大型协作系统；而 Nature 排除了功能的干扰，使目标简练而全面，能够保障业务对系统的**绝对控制权**，同时又能使功能实现不必瞻前顾后，顾虑重重。
- **数据净化**：传统方式开发很容易导致**业务数据，技术数据，控制数据和临时数据混杂在一起**，产生了不必要的耦合，且有时难以分离，无意中增加了系统的复杂性和维护成本；更为糟糕的是，关键业务数据可能被系统“绑架”，系统变得臃肿、低效且难以改变，而 Nature 的设计时模式完全由业务数据构成，没有一丁点的功能存在，保证了业务的纯粹性，直观性和简洁性。


## 如何使用

我们需要做下面的工作

1. 创建一个[mysql](https://www.mysql.com/) 或 [mariadb](https://mariadb.org/) 或 [Tidb](https://pingcap.com/en/) 数据库，并执行 [schema.sql](shell/schema.sql)

2. 在 [Meta](doc/ZH/help/meta.md) 数据表里定义多个业务对象，如：我们定义`订单`和`订单账`两个业务对象

   ```sql
   INSERT INTO meta (full_key, description, version, states, fields, config) VALUES
   ('B', 'sale/order', 'order', 1, '', '', ''),
   ('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}'); 
   ```

3. 在 [relation](doc/ZH/help/relation.md) 数据表使定义关系将多个业务对象关联起来，并在 `Relation` 里设置 `Executor` 用于业务对象间的转换（相当于流式计算中的 map），如上面的`订单`和`订单账`可以有这样的定义：

   ```sql
   INSERT INTO relation
   (from_meta, to_meta, settings)
   VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"states":{"add":["unpaid"]}}}');
   ```

4. 写代码实现您上面定义的 [Executor](doc/ZH/help/executor.md)。如果是基于 Http 的请在完成后启动它，如果是基于类库的请将之放到 与 nature.exe 相同的目录下。如对于`订单`和`订单账`来讲这个逻辑是：

    - 生成一个`订单账`业务对象
    - 从传入的`订单`中提取所有商品的应收款项记为该`订单账`的应收
    - 将`订单账`对象返回给 Nature 以驱动下一环节的处理

5. 配置好`.env` 文件中的`DATABASE_URL`属性以指向您创建的数据库

6. 启动 natrue.exe和retry.exe。

7. 对 Nature 发起 http post 请求，如将`订单`数据提交数据到 Nature，

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
