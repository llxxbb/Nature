# 内置模块

内置执行器用于[关系配置](relation.md)中，相对来讲内置执行器是非常通用的，内置于 Natrue 中以方便大家的使用。

随着 Nature 应用领域的不断发觉，内置执行器会不断进行丰富和完善，因此此文档可能会经常调整。

此文档将以字母顺序罗列这些组件。

## scatter 转换器

**作用**：将关系中的上游数据拆分成多条下游数据。并用 `Instance.para` 来区分这些下游数据

**上游数据要求**：`Instance.content` 的内容必须为下面的 json 形式， **提示**：数据格式如果不满足要求，可以使用关系定义中的 `filter_before` 来进行修正。

```json
[
  {
    "key": "a/1",
    "value": 33
  },
  {
    "key": "a/2",
    "value": 76
  },
  ...
]
```

**下游数据说明**：上游数据中的 key 所对应的值会成为下游数据的 `Instance.para`， 而 value 所对应的值则会被放入下游数据的 content 中。其在 instance 数据表的体现形式示例如下：

| ins_key                | content |
| ---------------------- | ------- |
| B:downstream:1\|0\|a/1 | 33      |
| B:downstream:1\|0\|a/2 | 76      |

**选项**：

| 选项                | 说明                                                         | 缺省值 |
| ------------------- | ------------------------------------------------------------ | ------ |
| dimension_separator | 如果上游数据中的 key 对应的值不是用 para 的分隔符进行分隔的，且想转换为 para 的分隔符以利于后续对 para 进行处理，则需要设置此属性。 如 key 对应的值是 “a-1”， 则此属性应设置为 “-”。 | /      |

## merge 转换器

**作用**：对上游多个数据合并，或将上游的一个数据与下游状态数据中的多个数据进行合并。

**注意**：sum 只能统计**整型**数据。

**选项**：

| 选项          | 说明                                                         | 示例  | 缺省值   |
| ------------- | ------------------------------------------------------------ | ----- | -------- |
| key           | 有两种模式：<br />Para：要统计的 key 位于`Instance.para`中，value 位于`Instance.content`中<br />VecTuple：key 和 value 都位于`Instance.content`中 | 见下  | VecTuple |
| when_same     | 遇到相同 key 时的处理模式：<br />Old：保留旧值<br />New：保留新值<br />Min：取最小的一个<br />Max：取最大的一个<br />Sum：取两者的和 | “Old” | “Sum”    |
| sum_all       | 是否需要在结果中添加 total 字段， 并将 所以的 key 值         |       |          |
| key_from_para |                                                              | [1,0] | []       |

**示例**：将上游和与下游的上一个版本的 `instance.content` 的值合并。



上游数据

| ins_key            | content |
| ------------------ | ------- |
| B:upstream:1\|0\|b | 76      |

下游的上一版本数据

| ins_key      | content                         | state_version |
| ------------ | ------------------------------- | ------------- |
| B:sum:1\|0\| | {"detail":{"a":37},"total":100} | 1             |

新生成的下游数据，其中 detail 中的 “b” 和 76 分别来源于本次上游数据的 `instance.para` 和 `instance.content`. 

| ins_key           | content                                  |
| ----------------- | ---------------------------------------- |
| B:upstream:1\|0\| | {"detail":{"a":37, "b":76}, "total":109} |

**结果说明**：detail 是所有的求和项，total 是所有 detail 中各项的和。detail 中的 key 可以通过 key_from_para 选项进行设置。

**模式**：sum 在求和过程中对 detail 中相同 key 的处理有很多模式，模式可以通过 when_same 选项进行说明。

**使用建议**：不适合对拥有大量不同 `detail.key` 的数据进行求和，因为每次求和都会生成一个版本数据，会形成大量的 IO。

## instance-loader 前置过滤器