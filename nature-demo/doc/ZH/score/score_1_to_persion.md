# 全员成绩单->个人成绩

## 成绩单

在本示例里，我们采用批量的方式将学生的成绩输入到 Nature.。输入的内容是一个二维数组，示例如下：

```rust
let mut content: Vec<KV> = vec![];
content.push(KV::new("class5/name1/subject2", 33));
content.push(KV::new("class5/name3/subject2", 76));
content.push(KV::new("class5/name4/subject2", 38));
content.push(KV::new("class5/name5/subject2", 65));
...
```

第一列说明了班级、学员和学科之间的关系，第二列则是成绩。源代码请参考：nature-demo::score::score_test。为了存储成绩单，我们需要定义元数据，如下：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'score/table', 'store original score data', 1, '', '', '');
```

让我们看一下运行效果：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib score::score_test
```

在demo 运行后，请检阅数据库的 instance 数据表，会有类似于下面的数据。：

| ins_key                                             | content                                                      |
| --------------------------------------------------- | ------------------------------------------------------------ |
| B:score/table:1\|f4c850bb749bd1bff135b578e428492e\| | [{"key":"class5/name1/subject2","value":33},{"key":"class5/name3/subject2","value":76},{"key":"class5/name4/subject2","value":38},{"key":"class5/name5/subject2","value":65}] |

`ins_key`的结构是 meta|id|para，因为我们没有指定 id, Nature会根据输入的数据取 hash 值作为其 id.

## 个人学科数据

接下来我们想要做的事情是，将上面这个成绩单拆分成一条一条的个人学科数据。以方便个人成绩查询，且杜绝学员之间相互串查。

- **Nature 要点**：其实`个人学科数据`的作用远不止于此，它会服务于后续的统计。我喜欢称这样的数据为**原子数据**，因为它足够小，足够细，可以组装成任何你想要的数据。它非常适合于流式计算这种层层递进进行统计的方式。

个人学科成绩的 `Meta` 定义如下：

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'score/trainee/subject', 'person original score', 1, '', '', '{"master":"B:score/table:1"}');
```

有了上面的`成绩单`和`个人学科成绩`两个**元数据**后我们就可以编织他们的关系了，并指定处理程序来完成转换工作。

```mysql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:score/table:1', 'B:score/trainee/subject:1', '{"executor":{"protocol":"builtIn","url":"scatter"}, "convert_after":[{"protocol":"http","url":"http://127.0.0.1:8082/add_score"},{"protocol":"localRust","url":"nature_demo:name_to_id"}]}');
```

我们先看 executor 的定义 ：

```json
{"executor":{"protocol":"builtIn","url":"scatter"}
```

- **Nature 要点**：`builtIn`是说我们不需要开发这个功能，直接拿来用就好了。Nature 内置了一些执行器，在后续的示例里我们将充分展示。与`自动执行器`不同，`内置执行器`需要在 `url` 属性里设置我们需要用到的功能，而`自动执行器`则不需要。
- **Nature 要点**：`scatter` 内置执行器的作用是，将`成绩单`中数据表格拆分成一条条独立的`个人学科成绩`。并将 表格数据的第一列放到`个人学科成绩`数据的`Instance.para` 里，而成绩数据则放到`Instance.content`中。请参考[内置执行器](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/built-in.md)

如果忽略`convert_after`的作用（不久我们会讲到），经过`scatter`处理后，`Instance`数据表中应该会看到下面的数据，

| ins_key                                             | content |
| --------------------------------------------------- | ------- |
| B:score/trainee/subject:1\|0\|class5/name1/subject2 | 33      |
| B:score/trainee/subject:1\|0\|class5/name3/subject2 | 76      |
| B:score/trainee/subject:1\|0\|class5/name4/subject2 | 38      |
| B:score/trainee/subject:1\|0\|class5/name5/subject2 | 65      |

既我们在上面输入的`成绩单`被拆成了4条`个人学科成绩`，而且`成绩单`的两列分别放到的 `int_key.para` 和 content位置。这里需要注意的是，**如果指定了para 而没有指定 id， Nature则会将id自动置为0，而不再是一个hash值**，所以这里你看到了 meta|0|para 这种形式。

## 运行Demo

先让我们来看一下真实的运行结果：

```shell
nature.exe
nature_demo_executor_restful.exe
cargo.exe test --color=always --package nature-demo --lib score::score_test
```

检索`instance`数据表中的数据我们会看到下面的结果：

| ins_key                                    | content |
| ------------------------------------------ | ------- |
| B:score/trainee/subject:1\|0\|001/subject2 | 37      |
| B:score/trainee/subject:1\|0\|003/subject2 | 80      |
| B:score/trainee/subject:1\|0\|004/subject2 | 42      |
| B:score/trainee/subject:1\|0\|005/subject2 | 69      |

`scatter`后的数据怎么会变成这种形式了呢？这就是 `convert_after` 的作用了。我们来详细讲解一下 `convert_after` 的作用，先看一下本示例我们给出的配置：

```json
"convert_after":[{"protocol":"http","url":"http://127.0.0.1:8082/add_score"},{"protocol":"localRust","url":"nature_demo:name_to_id"}]
```

- **Nature 要点**：`convert_after` 的作用是在`执行器`执行完后且在 Nature 保存数据前，对数据进行一些修正，特别适合于技术处理，如格式修正等。
- **Nature 要点**：`后置过滤器`可以由多个`过滤器` 构成，本示例定义了两个`过滤器`，一个是基于 http 方式调用，用于给所有参加学科2考试的人补分；一个是基于静态链接库调用，用于将 `班级/姓名`替换成学号。 这两个过滤器的实现请自行查看源代码，这里就不贴出来了。
- **Nature 要点**：每个`过滤器`的配置形式有点类似于`执行情`的配置形式，但其实现形式是不同的，具体请看源代码。
- **Nature 要点**：我们完全可以定义多个`Relation`来完成`后置过滤器`的功能，之所以使用`后置过滤器`是因为：
  - 性能：上面的 4 条数据是一次性被`后置过滤器`处理的，如果我们改用`Relation`的 `执行器` 来完成，对应的则需要定义两个`执行器`，而每个`执行器`只能一条一条地处理数据，这样我们就需要8次 IO 才能完成这个工作。性能不可同日而语。
  - `过滤器`一般是技术处理语义，而**`Relation`主导的是业务语义**，我并不希望向你的老板去理解这么一个技术性的“业务概念”。这同样适用于`前置过滤器`。