# 任务系统

Nature 在数据转换时，会生成任务数据，这些数据会存储与 `task`数据表中。

当所有的重试都失败或者遇到逻辑问题，任务将被转移到 `task_error` 数据表中以备日后排查。

任务系统主要有下面的职能：

## 记录重试次数

任务可能因为环境问题暂时无法执行，此时 Retry 模块会进行重试并记录重试的次数，每次重试的间隔时间会是上次的2倍。起始间隔时间请参考 .env 文件中的 `retry settings` 节点。

## 异常数据回放

当环境或逻辑问题修正后，异常的数据可以再次转移到 `task` 数据表中进行重做。

## 其他说明

有关重试的运行参数设计请参考 .env 文件中的 `retry settings` 节点。