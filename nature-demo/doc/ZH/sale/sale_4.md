# 销售额top

这是 Nature 最难解决的问题之一，我在上面花了很长时间，不过我花这么长时间就是为了节省您的时间，所以在这一小节里，您还可以继续享受到“无码”乐趣。

在这部分内容里我们还是用秒为单位进行统计，为了能够更好的理解这部分内容，您可以把统计单位由秒想象成天，而且一天有百万以上的订单需要处理。在这个基础上我们再来想如何算出销售额TOP问题。

## 定义统计任务

我们先来看第一组配置：

```mssql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/money/second_tag', 'top of money task' , 1, '', '', '{"cache_saved":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/item/money/second:1', 'B:sale/money/second_tag:1', '{"target":{"append_para":[0,1],"dynamic_para":"(time)"}}');
```

配置里没有新鲜元素，只是依据秒销售额数据生成了新的秒统计任务。请注意，我们之前也定义过一个秒统计任务：`sale/item/money/tag_second` ，两者的区别在于：先前的是针对给定商品的，而这里是针对所有商品的。

- **Nature 要点**：对于秒内所有商品的统计我们其实可以直接用`sale/item/money/second`来驱动，之所以用 `second_tag` 来驱动是因为同一目标数据 `sale/item/money/second` 可能会驱动多次。如果换做天为单位进行，可能会被驱动成千上万次，我们将会看到下面有一个比较恐怖的配置，而每一次驱动都会执行一次这个复杂的任务，所以能避免尽量避免。

另外说一点：上面这个关系中的 `sale/item/money/second` 完全可以换成 `sale/item/money/tag_second` 因为它们的实例除了 `Meta` 之外 para 是完全相同的。

## 销售额 Top

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/money/secondTop', 'top of money' , 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('L', 'sale/money/secondTopLooper', 'top looper' , 1, '', '', '{"multi_meta":["B:sale/money/secondTop:1"], "only_one":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/money/second_tag:1', 'L:sale/money/secondTopLooper:1', '{
"convert_before":[
    {"protocol":"builtIn","url":"task-checker","settings":"{\\"key_gt\\":\\"B:sale/item/money:1|0\\",\\"key_lt\\":\\"B:sale/item/money:1|1\\",\\"time_part\\":[0,1]}"},
    {"protocol":"builtIn","url":"task-checker","settings":"{\\"key_gt\\":\\"B:sale/item/money/tag_second:1|0|(time)/\\",\\"key_lt\\":\\"B:sale/item/money/tag_second:1|0|(time)0\\"}"},
    {"protocol":"builtIn","url":"task-checker","settings":"{\\"key_gt\\":\\"B:sale/item/money/second:1|0|(time)/\\",\\"key_lt\\":\\"B:sale/item/money/second:1|0|(time)0\\",\\"time_part\\":[0,1]}"},
    {"protocol":"builtIn","url":"instance-loader","settings":"{\\"key_gt\\":\\"B:sale/item/money/second:1|0|(time)/\\",\\"key_lt\\":\\"B:sale/item/money/second:1|0|(time)0\\",\\"page_size\\":1,\\"filters\\":[{\\"protocol\\":\\"builtIn\\",\\"url\\":\\"para_as_key\\",\\"settings\\":\\"{\\\\\\"plain\\\\\\":true,\\\\\\"part\\\\\\":[2]}\\"}]}"}
],"delay_on_para":[2,1],"executor":{"protocol":"builtIn","url":"merge","settings":"{\\"key\\":\\"Content\\",\\"sum_all\\":true,\\"top\\":{\\"MaxTop\\":1}}"}}');
```

先看一下元数据的定义：

-  `secondTop` 用于存放我们最终的统计结果

- **Nature 要点**：`secondTopLooper` 是一种新型的元数据：`MetaType::Loop`。Loop 类型的引入主要是为了应对分批次统计问题，我们假设要统计的量是百万、千万以上的数据，这么大的数据量是不可能一次加载并处理完成的。而 Loop 只是一个 `Meta`, 数据的加载工作还得依赖于下面定义的 `relation`。
- **Nature 要点**：Loop 只是个过渡型元数据，其目标元数据需要用 `multi_meta`属性给出。请参考：[meta.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/meta.md).

现在该看一下关系定义了，我承认这是一个非常反人类的展示，请原谅 Nature 目前还没有可视化的配置界面。但一想到因为不用写代码就可以完成任务，我们还是忍受一下吧。其实把它分解开来结构还是很清晰的。我们先看一下主体：

```json
{
	"convert_before":[...],
    "delay_on_para":[2,1],
    "executor":{"protocol":"builtIn","url":"merge","settings":"{\\"key\\":\\"Content\\",\\"sum_all\\":true,\\"top\\":{\\"MaxTop\\":1}}"}}
```

没错，我们又一次使用了 `merge` ，这至少证明它的通用性还是不错的。

- **Nature 要点**：为了能够演示出效果，这里只求 top 1，可依据实际情况进行修改。**注意**：如果上游数据量非常大，请不要使用 `top.None` 模式，该模式会记录所以商品的销售额，因为下游数据只是一条数据，其**容量有限**。 有关merge 请参考：[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)

关系里的上游数据只是一个时间标记而已，用于延时驱动（delay_on_para，前面讲过）本次的统计任务。所以我们还需要借助于 `convert_before` 来加载真正的待统计数据。然而这次的 `convert_before` 内容有点多。

```json
{"protocol":"builtIn","url":"task-checker","settings":".1."},
{"protocol":"builtIn","url":"task-checker","settings":".2."},
{"protocol":"builtIn","url":"task-checker","settings":".3."},
{"protocol":"builtIn","url":"instance-loader","settings":"..."}
```

- **Nature 要点**：task-checker 可以用于检测特定时间内的特定任务的状态，它检查的是 task 数据表的相关任务的状态。

我们完全可以基于 `sale/item/money`（单笔订单每个商品的销售额）来做 top N 统计，但考虑到我们已经对单品的秒区间做了汇总统计（`sale/item/money/second`），如果在这个基础上我们将节省很多算力。但这里有个问题，`sale/item/money/second` 处理是异步的，也就是说，我们要统计 top 时`sale/item/money/second` 数据很有可能没有准备好。

为此我们需要用 `task-checker` 来检查一下所有的 `sale/item/money` 任务是否完成。除此之外我们还要检查`sale/item/money/tag_second`和`sale/item/money/second`相关的任务，所以这里会有三个 `task-checker`。这三个 `task-checker`定义里只有第一个需要指定时间范围，其它两个的时间范围都被限定到task_key里了，所以不需要额外指定。

**注意**：其实这里对 `sale/item/money` 任务的检查是有缺陷的。因为我们是依据 Instance.create_time 来检查 task.create_time 或 task.execute_time。对于同一个`instance`来讲，这几个时间不太可能都落到同一个时间区间，尤其我们示例里使用秒作为统计区间，这会使问题会更严重。但我们的示例却运行的很成功，这是因为：

- 我们应用了 delay_on_para 进行了延时执行。
- 我们几乎遇不到网络抖动问题。

 top 统计一般用于**趋势分析**，多少少一点数据一般不会造成什么影响。而且在现实情况下，我们一般不会用小粒度的秒进行 top 统计，再加上延时处理（其设置值需要超过多次重试的时间），所以基本上可以杜绝漏统计的问题。如果要想对所有重试失败的已经过时任务重新统计，建议通过增加补偿 meta 的方式进行统计，然后在使用的时候将两者的统计结果进行合并就好，这里就不再给出具体示例了。

## 运行结果

```shell
nature.exe
retry.exe
cargo.exe test --package nature-demo --lib sale_statistics::sale_statistics_test
```

运行上面的程序，等几秒钟，我们就可以在 instance 数据表中看到类似于下面的数据产生了

| ins_key                                                    | content                            | sys_context                                                  |
| ---------------------------------------------------------- | ---------------------------------- | ------------------------------------------------------------ |
| B:sale/money/second_tag:1\|0\|1598068434/1598068435        |                                    | {"para.dynamic":"[[\"(time)\",\"1598068434/1598068435\"]]"}  |
| L:sale/money/secondTopLooper:1\|0\|1598068434/1598068435/1 | {"detail":{"1":7000},"total":7000} | {"loop.task":"{...},"loop.id":"1","loop.next":"B:sale/item/money/second:1\|0\|1598068434/1598068435/1\|0"} |
| L:sale/money/secondTopLooper:1\|0\|1598068434/1598068435/2 | {"detail":{"1":7000},"total":7300} | {"loop.task":"{...},"loop.id":"2","loop.next":"B:sale/item/money/second:1\|0\|1598068434/1598068435/2\|0"} |
| L:sale/money/secondTopLooper:1\|0\|1598068434/1598068435/3 | {"detail":{"1":7000},"total":7311} | {"loop.task":"{...},"loop.id":"3","loop.next":"B:sale/item/money/second:1\|0\|1598068434/1598068435/2\|0"} |
| B:sale/money/secondTop:1\|0\|1598068434/1598068435         | {"detail":{"1":7000},"total":7311} |                                                              |

- 第1条 `second_tag`是生成的秒数据统计任务， 注意一下 `sys_context` 中的 “para.dynamic”.

- 第2-4条是循环处理 top。因为演示的目的，我们将 `instance-loader `的 `page_size`=1 所以这里产生了多条数据，请留意 `sys_context` 中的 `loop.id` 和  `loop.next`的变化，这是Nature 的内部控制机制，大家做一下了解就可以了。
- 第5条是我们要的最终结果。detail 里放置的就是我们的 top 1，而 total 则放置的是当前秒内的所有销售额。

## 回顾

我们相对完整的演示了一些统计的关键应用情景，在此期间您可以看到除了数据格式转换需要用到代码外，其它问题我们全都是用内置执行器来解决的。而且在整个示例里我们只用了一次外部代码转换，其余的转换也是通过内置执行器来完成的。我不否认这些内置执行器是为构建演示而创建的，但如果您仔细评阅这些内置执行器的说明，您会发现它们是通用的，一个很好的例子就是 merge 内置执行器被用在了三个不同的地方。

我想说的是这些内置执行器加上这种处理模式可以真正的节省了您的代码，而不是仅能用于我设定的固定场景。也就是说 Nature 要解决的是真正的通用性问题，这会为大数据处理的标准化、简单化和规范化提供了基础保障并降低大数据的技术门槛。

