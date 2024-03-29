# Nature 的技术特性

Nature 目的就是让使用者聚焦于业务而非技术。为了到达易用性，Nature 封账了很多技术复杂性，以减轻技术人员的负担。

这其中就包括辣手的**数据一致性**问题，在传统方式下要想在多个系统间实现数据一致性是非常困难的。异构问题、网络问题、系统自身健壮性问题等都是数据一致性要考虑的内容，协作的系统越多问题越复杂。目前还没有见到有统一的、高效的、开箱即用的解决方案面世，这是萌生创作 Nature 的重要起因。

下面我们来看一下 Nature 都为我们提供了哪些技术能力：

## 数据的不可变性与幂等

Nature 只能插入数据不能变更数据，`Instance`一旦生成既被永久定格，这就防止变更导致的数据覆盖问题。这一特性使得 Nature 可以被信赖，因为生成的数据不可抵赖，且可以溯源。数据不可变的另外一个重要应用就是幂等。

为了实现幂等性，Nature 提供了以下措施和建议。

- 主键：`Instance` 数据表的主键构成为 ID + `Meta` + para + 状态版本。
- 预分配ID：在调用Nature 之前预先生成一个ID，或许 facebook 的 snowflake ID 生成器算法是一个不错的选择。使用此ID作为`Instance`的ID，这样当出现环境问题时使用相同的ID提交数据到 Nature 就不会存储多条数据了。如果你不提供ID，Nature 会使用哈希算法为你生成一个。
- para: 外部已经存在的数据导入 Nature 时 para 会很有用。此时 Para 可以是唯一标识外部数据的ID。如果使用 para，一般情况下 `Instance` 的 ID 置0。

Nature 是支持状态数据的，那么 Nature 如何保证数据不被修改？答案是版本。Nature 为状态数据的每一次状态变更都会生成一个新的`Instance`，但这些不同状态的`Instance`拥有相同的ID和`Meta`，只是版本号是不同的。如 [Demo](../../../nature-demo/README.md) 中的订单状态。

## 防重机制

Nature 需要面对下面情形所产生的问题：

- 并发冲突
- 环境变化

我们先看第一种情况，Nature 是**事件驱动**的，既然是事件，就无法确定触发的时机，就可能出现并发冲突问题。如出库单和入库单同时操作商品库存，很显然我们只能让一个成功，另一个失败。Nature 內建了版本冲突的控制，无需`Executor`进行干预，除非外部直接输入。其实现机制是这样的，如果下游数据是状态数据，Nature 在调用`Executor`之前先取出下游最近的状态数据并记录版本号，然后Nature 再调用`Executor`，当`Executor`返回状态数据后，Nature 会将之前记录的版本号+1 赋值给新返回的状态数据，当+1版本的数据已经存在时即可识别为冲突，冲突后 Nature 会再次获取最近的下游数据并再次调用 `Executor` 直到成功为止。冲突处理是Nature 內建的功能，`Executor`无需关注。

第二种情况也是比较常见的，如网络往往会有不稳定的情况，在此种情况下 Nature 会重试。但这里有一种业务情景的不稳定，如库存此时没有但下一时刻就有了。此时的控制权在 `Executor`， 如还想继续尝试，则返回环境错误，如果不想再次尝试则返回逻辑异常。

## 数据的一致性

数据一致性是系统在运行期间**维持数据关系正确性**的一种保证。我们一般寄希望于数据库的事务来保证，但业务系统的分布式特性，使得数据库的事务机制非常难以应用。这是一种技术门槛比较高的工作，很难有一种拿来即用的方案来应对，且可维护性比较差。

借助于重试与版本机制，Nature 实现了最终数据的最终一致性。这是Nature 内置的一种能力，使用者无需关心它的存在。

## 任务分发与`Instance`

调度的幂等性几乎遍及Nature的所有运行过程，这里我们讲一下任务分发。举一个例子：一个上游有两个下游跟随者，生成第一个下游时失败了，但第二个却成功了；这时候我们做了一个“危险”的操作，把第一个下游和上游的关系删除了；这时Nature正在重试失败的第一个分支，砰！相同的输入不同的输出！所以Nature 必须避免此类事情的发生。Nature 的做法是将关系产生的所有的任务数据都一同打包并一次性落盘，这样当关系改变时，就不会影响到已经生成的任务数据。

但是如果网络很糟的话，Nature 可能会重复生成任务数据，而这也有可能导致不幂等，所以任务数据本身也需要防重设计，防重的依据就是上游`Instance`的ID。

## `Executor`与`Instance`

有三种Executor：

- 前置Executor：在转换之前可以对上游数据进行编辑，如格式转换等。
- 核心Executor（或称之为转换器）：实现上游 Instance 到下游 Instance 的转换。
- 后置Executor：可对转换后的 Instance 进行编辑。

其实不引入前置、后置Executor也是可以的，完全可以使用多个`Relation`来解决。之所以引入前置、后置`Executor`是基于以下几点考虑的。

- 关系主要说明业务实体间的关系，具有业务语义。而前置、后置Executor一般是技术性处理，如果使其关系化，则由关系所呈现出来的业务图会不纯粹并令人费解。
- 从性能上来讲，前置、后置Executor作为中间结果不会落盘，因此要比`Relation`占用更少的资源。

Nature 是一个平台，它可能面对海量的数据和高并发的情景，在这种场景下最好的选择是使用分布式数据库。因为是分布式数据库，事务可能不被支持，在此种情况下如果`Executor`返回多个`Instance`，Nature 必须一条一条的保存这些数据，而这个过程可能被坏的网络环境打断，被打断的任务会被Nature 重新唤起，既`Executor`重新执行了一次任务，而Nature 不能要求`Executor`本身具有幂等性，于是问题出现了：`Executor`可能返回与上次不同的数据！

与任务分发一样，Nature 使用`任务`来解决这个问题，`任务`的内容包含了所有从`Executor`返回的`Instance`。Nature 在逐条保存`Instance`之前先保存这个任务。这样如果被打断，Nature 只需要从之前任务中取出所有的`Instance`重新保存一下就好了。

## 错误、回调

Nature 为`Executor`定义了两种类型的错误：

- `LogicalError`
- `EnvironmentError`

如果`Executor`遇到一个未定义的错误并且应该中断处理，它就可以返回一个`LogicalError`，接下来Nature 会将这个任务从`task`数据表转移的`task—error`数据表，并且不会尝试重新执行这个任务。

然而有些`Executor`因为执行时间很长，所以无论你重试多少次都无法成功，为此Nature 提供了回调机制来解决这个问题。当遇到这种情况时，`Executor`的实现者需要开启一个独立的线程去执行具体的任务，并立即返回一个异步处理信号及可能返回数据的时间给Nature，Nature 会依据此时间推迟下次重试的时间； 当`Executor`真正完成任务时，`Executor`的实现者需要主动调用Nature `callback`  接口并传入处理结果。

转移到`task——error`数据表中的任务都会记录失败的原因以便于使用者进行检查。

## 重试

Nature 在与`Executor`通信或者进行自身调度时会自动捕捉`EnvironmentError`。针对 `EnvironmentError` Nature 实现了一套机制来多次重试，当所有的重试都失败的时候，任务会从`task`数据表转移的`task—error`数据表。

重试可能会产生重复的`task`和`Instance`。如果检测到重复的  `task` 可以直接中断处理， Nature 有独立的重试模块会继续后面的处理。如果检测到重复的  `Instance` 则需要将原有的 `Instance`取出来替换掉当前的 `Instance`并继续后续处理，而不能像 `task` 那样中断处理，因为如果中断就无法形成后续的 `task`。

## 历史回溯

`Relation`可以构建出一张现在运行的业务网。但具体到某一笔业务，要想给出这笔业务是走的业务网中的哪一条或哪几条线路，对于`Relation`来讲是不合适的。Nature 用`Instrance`的`from`属性来解决这个问题，该属性记录了它的上游`Instance`。这样就可以非常方便的知道该笔业务的来龙去脉了。这对于传统业务系统来讲是件非常困难的事情，如接入一个性能和数据一致性无法保证的链路跟踪系统。

## 批处理

假设我们要统计一下一个火爆的电商网站的单品销售 top, 每次统计可能涉及到千万数据，传统的基于 sql 的统计已经不太现实。对于这个问题 Nature 提供了一套自己的解决方案。Nature 提供了一个专有的 `MetaType::Loop` ，Loop 可以**驱动**一次处理一批数据。有两种处理模式：

- MetaSetting.only_one = false

```
Upstream -> Loop + downstream
Loop -> Loop + downstream
...
Loop -> downstream
```

- MetaSetting.only_one = true

```
Upstream -> Loop
Loop -> Loop
...
Loop -> downstream
```

**注意**：对于 `MetaType::Loop` 来讲 `MetaSetting.only_one`如果设置为 true, Nature 会将要输出的 Instance 视为有状态的，只有这样才能实现结果的叠加，才能完成形如 input + old = new 这种形式的数据处理。但你不能把`MetaType::Loop` 的目标 Meta 设置为有状态的！因为从 Nature 外部来看我们只要一个最终结果而不是中间结果，如果置为状态数据会让人感觉到非常奇怪。为了实现这种效果，Nature会把中间结果作为 last_state 数据并带到下一个批次里处理直到完成为止。

批量的控制来源于 Nature 的一个[内置Executor](built-in.md)：`instance-loader` 后面有这样的示例，请参考：[示例及功能讲解](../../../nature-demo/README.md)。

## 上下文

上下文可提供额外的控制手段，如可通过上下文来编辑流程控制。上下文的另外一个好处是，使得业务数据更加纯粹，使得控制数据和业务数据完全分离。

上下文分为`系统上下文`和`用户上下文`。用户上下文是用户可以自行定义的，而系统上下文是 Nature 自身定义的。系统上下文在Nature 的功能构建上起到很重要的补充。如`MetaType::Loop` 和 `instance-loader` 的协作就用到了三个系统上下文：

- loop.next：用于控制下一个批次的开始 instance 条件
- loop.task：用于传递批数据的处理规则，只有第一个批次可以取得处理规则。
- loop.finished：标记所有批次是否处理完成。

除了这些外，还有用于桥接的系统上下文：`target.id` 和 `target.para`。当有 A->B->C的链路时，C想使用A的ID作为自己的ID,而B没有使用A的ID，这时候就需要B架一个桥了。当B为另一个体系的数据时会有这个问题。请参考：[示例及功能讲解](../../../nature-demo/README.md)。

还有用于动态参数替换的系统上下文：`para.dynamic`。一般我们在配置 Relation 数据时，都是定义好的固定内容。但有时候我们需要运行时确定一些参数，这时候就需要该上下文了。

## 可扩展性

### 业务的扩展性

`Meta` 可以通过版本技术来实现业务的变迁或发展。

### 技术扩展性（还未实现）

Nature 是面向业务的一个开发平台，并用简单的方式构建业务模型。它使技术和业务能够很好的解耦，这使得很多技术不用受限于具体的业务，同时又可以用统一而简单的方式来强化业务的能力，如监控、权限管理、可视化等。