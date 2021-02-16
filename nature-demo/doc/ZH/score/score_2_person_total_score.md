# 求出每个人的总成绩

现在我们来求每个人所有科目的`总成绩`。首先定义`Meta`

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'score/trainee/all-subject', 'all subject\'s score for a person', 1, '', '', '{"is_state":true}');
```

- **Nature 要点**：请注意 `config` 字段的值为空，因为在这里我们不需要任何状态，所以Nature 会将之视为非状态数据既常规数据来处理。然而个人成绩是一条一条汇总过来的，所以总成绩是在不断变化的，这就需要 `all-subject`是一个状态数据。为了达到这个目的，我们需要强制`all-subject`成为状态数据，这也是Nature 引入 `is_state` 属性的原因，此属性可以将任何非状态数据转换成状态数据。

有了`个人总成绩`的定义后，我们就可以进行计算了，建立下面的`Relation`

```mysql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:score/trainee/subject:1', 'B:score/trainee/all-subject:1', '{"target":{"append_para":[0]},"executor":{"protocol":"builtIn","url":"merge","settings":"{\\"key\\":{\\"Para\\":[1]},\\"when_same\\":\\"Old\\",\\"sum_all\\":true}"}}');
```

里面有几个点需要说明一下：

```json
"target":{"append_para":[0]}
```

`target`指的是 `B:score/trainee/all-subject:1`，`append_para` 指的是`B:score/trainee/subject:1`的 para 的哪个部分， 还记得吗，在上一节中这个para的形式是 “学号/学科”。整个的意思是说总成绩需要记录到 `B:score/trainee/all-subject:1|0|学号` 对应的`Instance`上。有关`copy-para`的说明具体请参考：[使用 Relation](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)

```json
"executor":{"protocol":"builtIn","url":"merge","settings":"{\\"key\\":{\\"Para\\":[1]},\\"when_same\\":\\"Old\\",\\"sum_all\\":true}"}
```

- **Nature 要点**：merge 内置执行器的作用是将上游 content 的值和下游的上一个版本的 content 中的 total 值进行相加并形成新版本的 total 值，具体请参考[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)

本节示例不需要任何代码，只需要配置一下就可以得到结果，运行下面的内容：

```shell
nature.exe
retry.exe
nature_demo_executor_restful.exe
cargo.exe test --color=always --package nature-demo --lib score::score_test
```

我们会在 instance 数据表中看到类似于下面的数据：

| ins_key | state_version | content |
| ------- | ------------- | ------- |
|B:score/trainee/all-subject:1\|0\|001|1| {"detail":{"subject2":37},"total":37} |
|B:score/trainee/all-subject:1\|0\|001|2| {"detail":{"subject2":37,"subject3":100},"total":137} |
|B:score/trainee/all-subject:1\|0\|001|3| {"detail":{"subject2":37,"subject3":100,"subject1":62},"total":199} |

我们可以清晰的看到 B:score/trainee/all-subject:1\|0\|001 这条数据共有3个版本。每个版本的content 都是增量的。

## 这不是最好的

**本示例仅限于有限计算结果的叠加**。其实这是一种低效的统计方法，因为每次叠加都会形成一个版本，这会消耗大量的IO资源，这对于高并发的电商销量统计而言显然是一种灾难。因此我们需要一种新的统计方法。请参考[销量统计demo](../sale/sale_1.md)

