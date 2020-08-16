# 内置执行器

通常情况下，执行器时需要开发人员介入的。对于一些常用的通用职能，Nature 进行了内置，以减轻开发人员的工作。有关执行器请参考[关系](relation.md)。

随着 Nature 应用领域的不断发觉，内置执行器会不断进行丰富和完善，因此此文档可能会经常调整。

## 转换器

### scatter 转换器

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

### merge 转换器

**作用**：对上游多个数据合并，或将上游的一个数据与下游状态数据中的多个数据进行合并。**注意**：merge 只能统计**整型**数据。

**配置**：

| 选项      | 说明                                                         | 示例      | 缺省值 |
| --------- | ------------------------------------------------------------ | --------- | ------ |
| key       | 有三种模式：<br />Para：统计项的 key 来源于上游 para。该模式将上游的一个数据与下游状态数据中的多个数据进行合并<br />None：统计项没有 key 值，该模式将上游的数字数组进行合并，`merge` 会对每个数字使用相同的 `ignore` 作为 key, 并在统计完成后丢弃 key<br />VecTuple：对上游多个数据合并，每个数据都是 [key, value] 结构 | 见下      | None   |
| when_same | 遇到相同 key 时的合并模式：<br />Old：保留旧值<br />New：保留新值<br />Min：取最小的一个<br />Max：取最大的一个<br />Sum：取两者的和 | “Old”     | “Sum”  |
| sum_all   | 是否需要在结果中添加 total 字段， 并将 所以的 key 值         | true      | false  |
| top       | MaxTop(u16) 倒序 topN<br />MinTop(u16) 正序 topN<br />None 不需要 top, | MaxTop(5) | None   |

**Para模式示例**：将上游的一个数据与下游状态数据中的多个数据进行合并。选项设置如下。

```json
{"key":{"Para":[0]},"sum_all":true}
```

这个配置会将上游 `Instance.para`中的第一个元素取出来作为要合并的 key，既下面 ins_key 所对应数据的 b。上游数据示例如下。

| ins_key            | content |
| ------------------ | ------- |
| B:upstream:1\|0\|b | 76      |

下游的上一版本数据

| ins_key      | content                         | state_version |
| ------------ | ------------------------------- | ------------- |
| B:sum:1\|0\| | {"detail":{"a":37},"total":100} | 1             |

合并好的下游数据如下：

| ins_key      | content                                  | state_version |
| ------------ | ---------------------------------------- | ------------- |
| B:sum:1\|0\| | {"detail":{"a":37, "b":76}, "total":109} | 2             |

因下游数据是有状态的，此示例不适合在拥有大量合并对象的场合下使用；一方面是容量问题，另一方面是性能问题。

**None模式示例**：对上游多个数据合并

上游数据示例如下。

| ins_key             | content       |
| ------------------- | ------------- |
| B:upstream:1\|123\| | ["1","3","5"] |

合并好的下游数据如下：

| ins_key        | content |
| -------------- | ------- |
| B:sum:1\|123\| | 9       |

**VecTuple模式示例**：对上游多个数据合并。选项设置如下：

```json
{"key":"VecTuple","when_same":"Old"}
```

上游数据示例如下。

| ins_key             | content                    |
| ------------------- | -------------------------- |
| B:upstream:1\|123\| | [["a":10],["b":2],["a":8]] |

合并好的下游数据如下：

| ins_key        | content         |
| -------------- | --------------- |
| B:old:1\|123\| | {"a":10, "b":2} |

如果配置中的 when_same 换成 Min 或 New 则 `instance.content` 的值将会是： {"a":8, "b":2}

## 前置过滤器

### time_range

**作用**：用于生成一个时间区间（单位：秒），并赋值给 `Instance.para` 属性。

**配置**：

| 选项      | 说明                                                         | 示例 | 缺省值 |
| --------- | ------------------------------------------------------------ | ---- | ------ |
| unit      | 区间尺度：s(econd), m(inute), h(our), d(ay), w(eek), M(onth), Y(ear) | 见下 | "s"    |
| value     | 如果 unit 的单位是 "s","m","h","d" 则 value 为间隔数（须 > 0）<br />如果 unit 的单位是 "w","M","y" 则 value 为偏移量; value 可以 < 0, 意味着从尾端向前端偏移。取值范围如下：<br />week : [-7, 6]<br /> month : [-20, 19]<br /> year : [-200, 199] | 5    | 1      |
| on_para   | 从哪里取用于计算的时间，如果为 true 则从上游的 `Instance.para`中取，否则取`Instance.create_time` | true | false  |
| time_part | 取上游的 `Instance.para`中的哪一个部分作为输入时间           | 1    | 0      |

生成示例的 ins_key 示例： B:sale/item/tag_second:1|0|1596207504/1596207505

### instance-loader 

**作用**：依据给定的条件检索出一批 Instance，并提取出所以的 content 属性形成数组，并替换掉 执行器入参中的 from.content。

**配置**：

| 选项      | 说明                                                         | 示例                                                         | 缺省值 |
| --------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------ |
| key_gt    | 形成SQL where 条件 ins_key > {key_gt}                        | B:sale/item/count:1\|0\|(item)/                              |        |
| key_lt    | 形成SQL where 条件 ins_key < {key_lt}                        | B:sale/item/count:1\|0\|(item)0                              |        |
| page_size | 一页数据的大小                                               | 20                                                           | 100    |
| time_part | 从上游 `Instance'para` 的哪两个取值作为开始和结束时间        | [0,1]                                                        |        |
| filters   | 是一个过滤器数组，提出出来的每个 `Instance.content` 都会顺序过滤处理一下，如进行数据格式修正。 | [ {"protocol":"localRust","url":"nature_integrate_test_executor:append_star"},     {"protocol":"localRust","url":"nature_integrate_test_executor:append_plus"} ] |        |

### task-checker

**作用**：在执行转换器之前，检测相关的 task 是否完成，一般用于批量加载 instance 时判断其是否全面就位。如没有就位则返回环境异常，以等待下次重试。

**配置**：

| 选项      | 说明                                                  | 示例                            | 缺省值 |
| --------- | ----------------------------------------------------- | ------------------------------- | ------ |
| key_gt    | 形成SQL where 条件 task_key > {key_gt}                | B:sale/item/count:1\|0\|(item)/ |        |
| key_lt    | 形成SQL where 条件 task_key< {key_lt}                 | B:sale/item/count:1\|0\|(item)0 |        |
| time_part | 从上游 `Instance'para` 的哪两个取值作为开始和结束时间 | [0,1]                           |        |
