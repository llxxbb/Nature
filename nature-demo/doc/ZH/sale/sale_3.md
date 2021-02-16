# 销售额统计

在上一节中我们生成了以时间为单位的区间统计任务，考虑到一个区间内的数据量有可能非常的大，比如以月为单位，此时我们将需要一些技巧了。对于这些技巧的支持不是 Nature 本身具有的，因为其普遍性，Nature 将之集成到 builtin 中，以方便大家的使用。

其解决方法其实很简单，就是各自统计各自的，这也是为什么 tag_second 的 `Instance.para` 包含一个商品ID的原因 。我们来看一下：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/money/second', 'second summary of money' , 1, '', '', '');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/item/money/tag_second:1', 'B:sale/item/money/second:1', '{"convert_before":[{"protocol":"builtIn","url":"instance-loader","settings":"{\\"key_gt\\":\\"B:sale/item/money:1|0|(item)/\\",\\"key_lt\\":\\"B:sale/item/money:1|0|(item)0\\",\\"time_part\\":[0,1]}"}],"delay_on_para":[2,1],"executor":{"protocol":"builtIn","url":"merge"}}');
```

我们一开始先定义了一个以秒为单位的单品销售额统计项。然后定义了一个`关系，这个关系的 settings 有点复杂，我们将之进行分解并一一说明。主要有两部分，主体部分为 内置转换器：merge，如下：

```json
{"convert_before":[],"delay_on_para":[2,1],"executor":{"protocol":"builtIn","url":"merge"}}
```

merge 主要统计秒内单品销售额。需要注意：

- **Nature 要点**：tag_second 只是个时间区间是没有数据的，在这里他的作用就是用于驱动统计任务的执行。而真正的数据加载时通过 convert_before 中定义的 `instance-loader` 来完成的。
- **Nature 要点**：时间区间数据创建完成后不能立即立即统计的，因为此时该区间有可能还没有结束，所以需要延时执行，这就是 `delay_on_para` 所发挥的作用。它的用意是要在 Instance.para 的某个部分上取一个时间（由`delay_on_para` 的第二个参数决定），并在此基础上延迟指定的时间（由`delay_on_para` 的第一个参数决定，既延时2s）。具体请参考[relation.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)
- **Nature 要点**：我们之前应用过 merge 一次，相较于学习成绩统计，这里使用了更高效的方法来对一批数据进行求和。merge支持多种统计模式，可以让你不用写代码就可以完成统计工作，详情请参考：[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)

```json
{"protocol":"builtIn","url":"instance-loader","settings":"{\\"key_gt\\":\\"B:sale/item/money:1|0|(item)/\\",\\"key_lt\\":\\"B:sale/item/money:1|0|(item)0\\",\\"time_part\\":[0,1]}"}
```

 `instance-loader` 是[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/build-in.md)中的前置过滤器。用于自动加载所需要的 Instance 数据。这里有一个要点，我们在运行时才能知道我们需要加载那些数据，如本例中的商品ID。这就需要用到参数替换功能了，如下：

- **Nature 要点**： (item)/ 和(item)0 中的“(item)”是要替换的参数，用于限定加载哪个商品的待统计数据。其中（item）在运行时很会被 sys_context 中指定了 para.dynamic.(item) 的值替换掉。

instance-loader 的 key_gt 和 key_lt 用于限定 Instance 数据表中 ins_key 的范围，time_part 则是从上游Inspance.para相应部分获取时间信息并用于限定 Instance 数据表中 create_time的时间范围，可参考[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)。

好了，我们本示例的工作就结束了，没有代码，让我们看下运行结果，执行下面的命令：

```shell
nature.exe
retry.exe
cargo.exe test --package nature-demo --lib sale_statistics::sale_statistics_test
```

结果类似于下面的数据：

| ins_key                                                | sys_context |
| ------------------------------------------------------ | ----------- |
| B:sale/item/money/second:1\|0\|1596367993/1596367994/3 | 11          |

我们在此时间对3好商品共完成两笔交易，一笔是6元，一笔是5元，所以销售额一共是 11 元，大家可以在 sale_statistics_test 的提交代码中看到这一切。

下面我们将面对更大的挑战：[销售排行统计](sale_4.md)。