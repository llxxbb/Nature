# Nature interface

The interfaces provided by Nature are all based on http requests.

## /input

This is the main method for external systems to input information to Nature.

The input parameter is `Instance`, please see `Instance` in [Data definition](data-define.md). If the caller does not give an `Instance.id` and an `Instance.para` is not specified, Nature will assign an id to it.

The output parameter is `Instance.id` and the type is Result<u64>. The json example shown as follows.

```json
{"Ok":12345}
```

## /callback

`callback` is used for the `convert` interface of [Executor](executor.md) to communicate with Nature asynchronously. In this way `convert` must immediately return `ConverterReturned::Delay(seconds)` defined in [Data Definition](data-define.md) to tell Nature that the result will be returned within the time given by `Delay`, when After completing the task, `convert` pushes the result to Nature by calling Nature's `callback` interface. If the result does not submit within the `Delay` time, Nature will retry according to retry strategy.

For input parameter, please refer to `DelayedInstances` in [Data Definition](data-define.md), output parameter is Result<()>, and its json format is as follows:

```json
{"Ok":null}
```

## /get/byId

Used to query the `instance` of the given [Meta](meta.md) and id.

For parameter reference, please refer to `KeyCondition` of [Data Definition](data-define.md), output parameter is Result<Option<Instance>>, and the corresponding json format is as follows:

**exist**

```json
{"Ok":{}} // {} is instance, please refer to Instance in the data definition
```

**does not exist**

```json
{"Ok":null}
```

## /get_by_key_range

Used to query the batch of `Instance`s.

Please refer to the `KeyCondition` of [Data Definition](data-define.md) for input reference, output parameter is Result<Vec<Instance>>, and the corresponding json format is as follows:

```json
{"Ok":[]} // [] is instance array, please refer to Instance in the data definition
```

## /task/redo

This interface is the internal interface of the Nature system, you only need to understand it, you will not use this interface directly. This interface used to retry failed tasks and is called by the `retry` executable program.