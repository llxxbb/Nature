# 内置 Executor

通常情况下，`Executor` 需要开发人员进行实现。对于一些常用的通用职能，Nature 进行了内置，以减轻开发人员的负担。有关 `Executor` 请参考 [Executor](executor.md) 和 [Relation](relation.md)。

随着 Nature 应用领域的不断发觉，内置 `Executor` 会不断进行丰富和完善，因此此文档可能会经常调整。

在 [Executor](executor.md) 中的定义了三种 `Executor` : `converter`， `convert_before` 和 `convert_after`，下面的内容按照这三种接口进行组织。

## converter 接口

### scatter

**作用**：将关系中的上游 Instance 拆分成多条下游 Instance。并用 `Instance.para` 来区分这些下游 Instance。

**上游 Instance 要求**：`Instance.content` 的内容必须为下面的 json 形式， **提示**：数据格式如果不满足要求，可以使用 `convert_before` 来进行修正。

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

**下游 Instance 说明**：上游数据中的 `key` 所对应的值会成为下游 Instance 的 `Instance.para`， 而 `value` 所对应的值则会被放入下游的 `Instance.content` 中。如上面 json 将产生下面的 `Instance` ：

| meta           | para | content |
| -------------- | ---- | ------- |
| B:downstream:1 | a/1  | 33      |
| B:downstream:1 | a/2  | 76      |

**选项**：

| 选项                  | 说明                                                                                             | 缺省值 |
| ------------------- | ---------------------------------------------------------------------------------------------- | --- |
| dimension_separator | 如果上游 `Instance.content` 中的 key 值不是 `Instance.para` 的分隔符则需要设置此属性。 如 key 值是 “a-1”， 则此属性应设置为 “-”。 | /   |

### merge

**作用**：对上游多个数据合并，或将上游的一个数据与下游状态数据中的多个数据进行合并。**注意**：merge 只能处理**整型**数据。

**选项**：

| 选项        | 说明                                                                                        | 示例        | 缺省值   |
| --------- | ----------------------------------------------------------------------------------------- | --------- | ----- |
| key       | 有三种模式：Para，None和Content。下面会具体说明                                                           | 见下        | None  |
| when_same | 如何处置相同的 `key`：<br />Old：保留旧值<br />New：保留新值<br />Min：取最小的一个<br />Max：取最大的一个<br />Sum：取两者的和 | “Old”     | “Sum” |
| sum_all   | 是否需要在结果中添加 total 字段。                                                                      | true      | false |
| top       | MaxTop(u16) 倒序 topN<br />MinTop(u16) 正序 topN<br />None 不需要 top,                           | MaxTop(5) | None  |

#### Para模式

将上游 `Instance` 合并到 下游 `Instance` 中，有下面的要求：

- 下游 `Instance` 的 `Meta` 须是有状态的，用于存储合并的结果。因下游数据是有状态的，此模式不适合在拥有大量合并对象的场合下使用；一方面是下游本身容量问题，另一方面是性能问题。
- 合并项的 `key` 来源于上游 `Instance.para`
- 合并项的 `value` 来源于上游 `Instance.content`

示例设置如下：

```json
{"key":{"Para":[0]},"sum_all":true}
```

上游示例数据如下，根据示例设置则 `key` 为 b，`value` 为76。

| meta-string  | para | content |
| ------------ | ---- | ------- |
| B:upstream:1 | b    | 76      |

下游示例数据如下：

| meta-string  | content                         | state_version |
| ------------ | ------------------------------- | ------------- |
| B:sum:1\|0\| | {"detail":{"a":37},"total":100} | 1             |

合并好的下游数据如下：

| meta-string | content                                  | state_version |
| ----------- | ---------------------------------------- | ------------- |
| B:sum:1\    | {"detail":{"a":37, "b":76}, "total":112} | 2             |

#### None模式

统计项没有 key 值，该模式将上游的数字数组进行合并，并将结果作为下游 `Instance` 输出。

上游数据示例如下。

| meta-string  | id  | content       |
| ------------ | --- | ------------- |
| B:upstream:1 | 123 | ["1","3","5"] |

合并好的下游数据如下：

| meta-string | id  | content |
| ----------- | --- | ------- |
| B:sum:1     | 123 | 9       |

#### Content模式

对上游多个数据合并，每个数据都是 [key, value] 结构

选项示例设置如下：

```json
{"key":"Content","when_same":"Old"}
```

上游示例 `Instance` 如下。注意`content` 为字符串数组，每个字符串又是一个只有两个元素的数组，第一个为key，第二个为值。

| meta-string  | id  | content                                      |
| ------------ | --- | -------------------------------------------- |
| B:upstream:1 | 123 | ["[\\"a\\":10]","[\\"b\\":2]","[\\"a\\":8]"] |

合并好的下游数据如下：

| meta-string | id  | content         |
| ----------- | --- | --------------- |
| B:old:1     | 123 | {"a":10, "b":2} |

如果设置中的 `when_same` 值将 Old 换成 Min 或 New 则 `instance.content` 的值将会是： {"a":8, "b":2}

## convert_before 接口

### para_as_key

**作用**：将 `Instance.para` 的一部分作为 key, 将 `Instant.content` 作为 value 组成 [key, value] 形式替换掉原有的 `Instance.content`。

**选项**：

| 选项    | 说明                              | 示例   | 缺省值   |
| ----- | ------------------------------- | ---- | ----- |
| plain | 如果为 false 则 `value` 的值会放到双引号中去。 | true | false |
| part  | 取上游的 `Instance.para`中的哪几个部分作为输入 | [1]  |       |

**示例**：

Instance.para = "ll/xx/bb"

Instance.content = 123

如配置为

```json
{
    "plain":true,
    "part":[1]
}
```

执行后， 则 Instance.content 变为： ["xx",123]

如配置为

```json
{
    "part":[0,2]
}
```

执行后， 则 Instance.content 变为： ["ll/bb","123"]，注意123加了双引号

### time_range

**作用**：用于生成一个时间区间（单位：秒），并赋值给 `Instance.para` 属性。

**选项**：

| 选项        | 说明                                                                                                                                                                                                        | 示例   | 缺省值   |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- | ----- |
| unit      | 区间跨度：s(econd), m(inute), h(our), d(ay), w(eek), M(onth), Y(ear)                                                                                                                                           | 见下   | "s"   |
| value     | 如果 `unit` 的值是 "s","m","h","d" 则 `value` 为间隔数（须 > 0）<br />如果 `unit` 的值是 "w","M","y" 则 `value` 为偏移量; value 可以 < 0, 意味着从尾端向前端偏移。取值范围如下：<br />week : [-7, 6]<br /> month : [-20, 19]<br /> year : [-200, 199] | 5    | 1     |
| on_para   | 从哪里取用于计算的时间，如果为 true 则从上游的 `Instance.para`中取，否则取`Instance.create_time`                                                                                                                                    | true | false |
| time_part | 取上游的 `Instance.para`中的哪一个部分作为输入时间                                                                                                                                                                         | 1    | 0     |

生成para示例：1596207504/1596207505

### instance-loader

**作用**：依据给定的条件检索出一批 `Instance`，提取出所有的 `content` 属性形成数组，并替换掉 `Executor` 入参[ConverterParameter](data-define.md) 中的 `from.content`。

**选项**：

下面用到的 ins_key 构成形式为：meta-string:id:para.

| 选项        | 说明                                                   | 示例                                                                                                                                                               | 缺省值 |
| --------- | ---------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | --- |
| key_gt    | 形成SQL where 条件 ins_key > {key_gt}                    | B:sale/item/count:1\|0\|(item)/                                                                                                                                  |     |
| key_lt    | 形成SQL where 条件 ins_key < {key_lt}                    | B:sale/item/count:1\|0\|(item)0                                                                                                                                  |     |
| page_size | 一页数据的大小                                              | 20                                                                                                                                                               | 100 |
| time_part | 从上游 `Instance.para` 的哪两个部分取值作为开始和结束时间                | [0,1]                                                                                                                                                            |     |
| filters   | 是一个`Executor`数组，每个都是convert_before 接口的定义，用于数据加载后的处理。 | [ {"protocol":"localRust","url":"nature_integrate_test_executor:append_star"},     {"protocol":"localRust","url":"nature_integrate_test_executor:append_plus"} ] |     |

### task-checker

**作用**：在执行 `Executor` 之前，检测相关的 Nature 任务是否完成，一般用于批量加载 instance 时判断所需的数据是否全面就位。如没有就位则返回环境异常，以等待下次重试。

**选项**：

| 选项        | 说明                                    | 示例                              | 缺省值 |
| --------- | ------------------------------------- | ------------------------------- | --- |
| key_gt    | 形成SQL where 条件 task_key > {key_gt}    | B:sale/item/count:1\|0\|(item)/ |     |
| key_lt    | 形成SQL where 条件 task_key< {key_lt}     | B:sale/item/count:1\|0\|(item)0 |     |
| time_part | 从上游 `Instance.para` 的哪两个部分取值作为开始和结束时间 | [0,1]                           |     |
