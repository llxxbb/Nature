# Guide

## implement converter

**Input Parameter**

// TODO


## callback

**Input Parameter**

* carrier_id

// TODO

## Batch input

### parallel process
 
will ignore error while store instance.

### serial process

In this case, it should has a converter to process after all items in batch had stored.

Important to know the converter next to serial store:

* source must be "/S/serial"
* give a specific context name for "/S/serial"
  * which will be bind to "/S/serial" `Instance` by Nature.
  * Nature use this context name to select but your converter not other converter.
  * Advice: give the target biz name as this context name. 
The given context will contains two list as a Json:
```json
{
    succeeded_id: Vec<UuidBytes>,
    errors: Vec<String>,
}
```

支持业务ID
可以大幅度简化业务端与Datahub集成的复杂度

## 注意事项

### `组`和`权重`的使用

引入`组`和`权重`的意义：业务逻辑需要调整，但对于风险控制来讲，需要有“灰度”发布功能。`组`和`权重`的引入就是用来解决灰度发布的问题。

使用灰度发布的情景：

- 目标数据定义相同，处理方式有调整。
    此情况下，不同的执行器位于相同的组（**不需要明确指定**）内，每个执行器依据权重值得到一个执行概率。
- 目标数据发生改变，
    此情况下，多个不同转换配置共享一个组（**需要明确指定**），每个执行器依据权重值得到一个执行概率。

**注意**对于一个 A->B 的数据定义，只能有**不多于一个**的转换器被执行，既转换器内只能定义一个`组`。
