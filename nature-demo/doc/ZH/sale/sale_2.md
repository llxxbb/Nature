# 定义统计区间

在本示例里我们将使用批量处理模式来解决上一个 demo (学习成绩统计) 中的性能问题。而且这种统计方式**不需要用到状态数据**。

对于一个销量火爆的在线销售系统来讲，业界常规的做法是按时间区间进行统计，这样可以及时了解商品的销量情况。所以我们也按这种方式进行统计，来看下 `meta` 和 `relation` 的定义。

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/money/tag_second', 'time range for second' , 1, '', '', '{"cache_saved":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/item/money:1', 'B:sale/item/money/tag_second:1', '{"target":{"append_para":[0],"dynamic_para":"(item)"},"executor":{"protocol":"builtIn","url":"time_range"}}');
```

为了简单起见我们只对销售额进行统计。我们为商品的销售额都配置了一个 `tag_second` 的 `Meta` 用于保存时间区间信息，这个区间信息是依据单条商品统计的入库时间求得的，这个可以通过[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)： time_range 来自动为我们完成。我们来看下配置里的新元素：

- **Nature 要点**：`cache_saved` 会 让 Nature 暂时记住已经写入的 `Instance` ，以避免重复写入。这在大并发请情境下会极大的提升性能。对于本示例来讲，这种情况会发生在 tag_second 身上（请留意日志中的 cached key: B:sale/money/second_tag 字样）。**危险提醒**：这个选项不是必须的，如果用错了反而会有很大的负作用。详细请看：[meta.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/meta.md)
- **Nature 要点**：`time_range`  是一个内置执行器。用于为下游`Instance`自动生成带有时间范围的 `para` 。这里依据上游 `Instance` 的创建时间来确定时间范围。具体请参考[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)中的 time_range。
- **Nature 要点**：target.append_para 是在目标 `instance.para` 上追加一个 para， 这个 para 来源于上游 para 的某个部分，在本例中是 sale/item/money 的 item 部分。append_para 具体请参考[relation.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)
- **Nature 要点**： `target.dynamic_para` 需与 append_para 一起连用，append_para提取得 item 保存到 sys_context 的`para.dynamic`属性中，并命名为“(item)”。 para.dynamic 的作用是替换下游任务中的(item)参数（请见[单品统计](sale_3.md)）。**注意**：目前 `para.dynamic` 只支持简单的替换，建议添加明确的边界符，如本示例用"()"，以避免发生错误的替换。dynamic_para具体请参考[relation.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)

让我们执行下面的命令来看一下运行结果：

```shell
nature.exe
cargo.exe test --package nature-demo --lib sale_statistics::sale_statistics_test
```

结果类似于下面的数据：

| ins_key                                                    | sys_context                             |
| ---------------------------------------------------------- | --------------------------------------- |
| B:sale/item/count/tag_second:1\|0\|1596367993/1596367994/2 | {"para.dynamic":"[[\"(item)\",\"2\"]]"} |
| B:sale/item/money/tag_second:1\|0\|1596367993/1596367994/3 | {"para.dynamic":"[[\"(item)\",\"3\"]]"} |

我们可以看到 `time_range` 所生成的 para 都已经附加到对应的 `meta` 上了，并且商品ID也被添加到了最后。 其形式是：开始时间/结束时间/商品ID。现在我们需要进入到下一个环节:[单品统计](sale_3.md)。