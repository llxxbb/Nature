# Nature 接口定义

Nature 所提供的接口都是基于 http 请求的。

## /input

这是外系统输入信息到 Nature 的主要方法。

入参为 `Instance` ,请见[数据定义](data-define.md)的 Instance。如果调用者没有给出 `Instance.id`且未指定 `Instance.para` 则 Nature 会为其分配一个 id。

出参为 Instance.id，是16进制的字符串， 类型为 Result<String>。其 json 示例如下

```json
{"Ok":"76c98eb5f5c524b2"}
```

## /callback

callback 用于 [Executor](executor.md) 的 convert 接口和 Nature 进行异步通讯，异步方式下 Nature 无需等待 Executor 的结果，当 convert 接口完成任务后通过 callback 接口主动将结果推送给 Nature。

入参请参考[数据定义](data-define.md)的 `DelayedInstances`，出参为 Result<()>，其 json 形式如下：

```json
{"Ok":null}
```

## /get_by_id

用于查询 instance 业务实例。

入参请参考[数据定义](data-define.md)的 `KeyCondition`， 出参为 Result<Option<Instance>>，对应的 json形式如下：

存在

```json
{"Ok":{}}	// {} 为 instance, 请参考数据定义中的 instance
```

不存在

```json
{"Ok":null}
```

## /get_by_key_range

用于查询 instance 业务实例。

入参请参考[数据定义](data-define.md)的 `KeyCondition`， 出参为 Result<Vec<Instance>>，对应的 json形式如下：

```json
{"Ok":[]}	// [] 为 instance 数组, 请参考数据定义中的 instance
```