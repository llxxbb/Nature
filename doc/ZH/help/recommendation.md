# 使用建议

## context 和 para 的区别
  - context 是K-V 而 para 只有 V
  - para可以唯一标记一个 `Instance` 而 context 没有这种属性。
  - 对 Nature 而言 para 不能用于流程控制， 而 context 则可以。

## 流程编排方法

- 避免构成死循环，
- 不建议用 `关系.执行器`来处理技术语义的流程，如数据格式的转换等，可以考虑用 `关系`的`前置过滤器` 和 `后置过滤器`来处理,这样不会破坏`关系`的业务语义,而且有可能提升性能。

## 实例生成

### 下游实例ID

尽可能避免使用`sys_context."target.id"`这是编程方式来控制下游ID。`use_upstream_id`、`Meta.master`和 `id_bridge` 都是以配置的方式来控制下游ID的生成。`Meta.master` 可以影响多个`关系`而`use_upstream_id`只能影响一个，`id_bridge`则可以在中断ID的地方进行搭桥 。

使用`Meta.master`：除了生成下游ID外，Nature 还会将上游的master 一并传递给`执行器`，这一点在下游单纯是状态数据时会非常有用。

### 生成具有不同 MetaType 的多个实例

可以使用 `MetaType::Multi`来作为下游的输出。请参考 [demo](https://github.com/llxxbb/Nature-Demo) 中的 销售统计部分

### 重复生成相同实例

如果会重复生成相同实例，请设置[`Meta`](meta.md) 的 cache_saved为 true  会大幅度提升性能。

## 海量数据统计

如需要进行海量数据的统计，请考虑使用 `MetaType::Loop` （参考[meta.md](meta.md)）并结合 [内置过滤器](built-in.md) `instance-loader`来实现。请参考 [demo](https://github.com/llxxbb/Nature-Demo) 中的 销售统计部分

## 避免多次触发重型任务

下游如果是归结性的计算量比较大的任务，最好将上游“归一”后在后接这个下游任务，这样可以避免无谓的重复结算！

## 如何修复不可变更的数据

Nature 没有变更数据的能力，如果数据真的有问题，可以采用对冲的方法进行修正。

