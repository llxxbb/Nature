# Introduce

There are all **Things** in Nature, and **Things** always transform into other things over time.

## ppt 的背景

以黑底白字为主：

韵味：真理就是简单明了。

如星空（广阔、深邃，未知）。

自然：就是世间的万物及其之间的联系。

## 业务系统的痛点
    业务定义的约束性。
    业务的最高抽象是一个“点”，属性和功能是从属的。现行的业务系统是没有这个抽象的，有也是局部的。够不成“管理树”
    管理树。业务点间的联系
    
### 点

    这是个非常重要的概念。
    在宇宙中星系是点的存在
    在太阳系中星球是点的存在
    在常规的物质中分子、原子是点的存在
    在原子中粒子是点的存在
    ……
    你可以发现点是事物的最好抽象。它将世界的复杂性用两个东西就可以表述：点和点之间的联系。   
    
## Things（点）

空间：space

组织方式：宇宙->星系->...->代码块{}

## motion

运动产生变化

### 演进

流程不是一蹴而就的，而是一个不断演化的过程。如何快速的迭代系统？
成长过程

订单的处理工程

### 分叉

细胞分裂（分叉）

订单拆分

订单与用户、商家的对应关系。

###  融合

受精（融合）

汇总统计。装配

### 时间轴

 过去-> 现在 -> 将来

 进化论，人类进化图

### 历史不可改变

 我们只能看到事务某一时刻的样子。
 
### 自我治理

## Nature

things + motion = nature

## theory

### retry

generate new tasks and finished older task, when it was broken retry it.

### idempotent

If instance or task exists new comers will be dropped.
When instance changed, Nature don't update it but version it, so you can have a whole changes list of history

## 技术意义

* 去接口
数据就是接口。接口只是一种技术形式。我们真正关心的是数据。因此我们不需要定义接口只需要定义数据！

* 內建状态数据版本冲突机制
* 异步回调
* 批处理
* 并行处理与串行处理
  - 并行中的串行
  - 串行中的并行
  
* 延时与定时执行

## any db you want

你缺少的元数据管理

一阴一阳谓之道
  thing 为因
  flow 为阳

