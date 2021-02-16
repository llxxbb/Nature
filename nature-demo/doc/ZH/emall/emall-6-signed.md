# 签收

这是订单处理流程的最后一步：签收。签收是一件不受控的事情，物流公司并不会主动将签收信息反馈给我们，用户也不一定会及时登录到我们的系统上来签收。那么我们怎么完成这些订单呢?一个可行的方法是，我们等待两个星期，如果这期间没有投诉，我们就自动签收它。

好了，开始行动，我们需要先定义一个用于接收签收信息的`meta`:

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/orderSign', 'order finished', 1, '', '', '{}');
```

对于主动签收的情况，这里没有提供代码，实现起来应该非常简单，除了签收信息本身外别忘了在`sys_context`放置 `target.id` 就好，因为我们后面要更新订单的状态。

对于自动签收来讲，我们还需要定义一个关系：

```mysql
-- orderState:dispatching --> orderSign
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:sale/orderSign:1', '{"delay":1, "id_bridge":true, "selector":{"state_all":["dispatching"]}, "executor":{"protocol":"localRust","url":"nature_demo:auto_sign"}}');
```

有关`auto_sign`的内容请自行参考源代码。

- **Nature 要点**：`delay`属性告诉Nature 不要让执行器立即执行任务，而是要等待指定的时间后再执行。在本例里这个任务就是自动签收。因为这只是一个 Demo ，为了我们的时间着想，我们将两星期压缩到1s，这样你就能够很快的看到签收的结果。
- **Nature 要点**：请注意，我们又一次用到 `id_bridge` 这就意味着下一个关系（`orderSign --> orderState:signed`）将很容易被处理，是的你将**又一次见证不用写代码的奇迹**。

接下来更新我们的订单状态：

```mysql
-- orderSign --> orderState:signed
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderSign:1', 'B:sale/orderState:1', '{"target":{"state_add":["signed"]}}');
```

到最后时刻了，请运行下面的内容：

```shell
nature.exe
nature_demo_executor_restful.exe
retry.exe
cargo.exe test --color=always --package nature-demo --lib emall::emall_test
```

- retry.exe：因为执行器不能立刻执行，Nature 会放弃对它的处理，为了唤起这些被挂起的任务，你需要启动 `Nature-Retry`

 让我们看下运行结果：

| ins_key                                                | content                                             | states     | state_version | sys_context                                     | from_key                                                  |
| ------------------------------------------------------ | --------------------------------------------------- | ---------- | ------------- | ----------------------------------------------- | --------------------------------------------------------- |
| B:sale/orderSign:1\|1954acea643ba7b380325bd4fd9c9b84\| | type=auto,time=2020-06-14 09:10:37.957013700 +08:00 |            |               | {"target.id":"3827f37003127855b32ea022daa04cd"} | B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\|\|5 |
| B:sale/orderState:1\|3827f37003127855b32ea022daa04cd\| |                                                     | ["signed"] | 6             |                                                 | B:sale/orderSign:1\|1954acea643ba7b380325bd4fd9c9b84\|\|0 |

## Demo 总结

至此我们的这个 demo 就结束了。我得承认我刻意将示例做的简单，也许你的业务会比这个复杂很多，但我想说的是你的业务越复杂，Nature 就会为你做越多，通过本示例可以发现 Nature 几乎在所有环节都尽可能的去除了技术复杂度，大幅度减少项目的工程量，甚至是业务代码，如本 demo 中有关支付状态和订单状态的处理，这对项目的稳定和可维护性是非常重要的。然而这不是 Nature 的全部，下面让我们具体来看一下：

- **配置既需求**：
  - **以可视的结果（`meta`）为单位**：能够简单、清晰、具体的描述需求。需求不会过大需要拆分，也不会过于具体导致缺少抽象，不会产生多余的赘肉，也不会遗漏关键节点。总之 `meta` 会让你恰到好处的找到你要的点。杜绝各个环节的翻译不准确性，杜绝各种不需要的支持数据或临时数据，快速提炼系统的核心价值。
  - **团队认知一致性**：因为`meta`足够简单、清晰、具体，团队中的各个角色非常容易达成共识，大幅度提升沟通效率。
- **配置既设计**：
  - **流程控制**：用配置代替编码，尤其是状态处理的配置化，大幅度降低业务开发维护成本。
  - **对实施进行强约束**：有力保障设计不偏离需求，不打折扣实现需求。实施和设计不匹配的事情在这里不会发生，也就避免了出现技术债务。
  - **动态设计**：传统设计是一种**静态设计**，一般会被固化到代码中，所以设计变更会比较困难，在一些关键的点上甚至不敢变更。但Nature 将设计和实现完全分离，完全没有这些问题。而且变更是有版本的，这就不会对既有的设计产生任何不良影响。这样就可以快速响应需求的变化。

- **简易性，稳定性，可维护性，可扩展性**：Nature 基本上用执行器来聚合外部的业务逻辑，这是一个统一的口，所以 Nature 就可以对这个口应用 AOP（Aspect Oriented Programming）技术，在外部无感知的情况下增强系统的能力，而 Nature 做的越多，使用者就会越轻松。表现在以下几个方面：
- **技术兜底**：开发人员现在不用关心并发、冲突处理、重入、幂等、重试和延迟执行等技术问题。减少了项目的技术门槛并提高可维护性。
  - **自动化业务流程**：封装了状态处理等业务功能，状态处理在传统方式下是非常复杂和难以维护的，而在 Nature 中现在一般场景下无需编码就可以实现。
  - **提高执行器的复用性和规范性**：因为 Nature 对数据形式的统一，这会对处理模式相同的执行器的统一提供了基础。后面的 Demo 会涉及到统计，里面用到很多 Nature 内置的执行器，这样你基本上不用写代码就可以实现统计功能了。
- **其它**：
  - **业务追溯**， 你想知道一笔业务的来龙去脉，直接看 from_key 就可以了，不需要麻烦程序员了，程序员也不需要看日志了。
