# versus other solutions

## map-reduce framework(MRF)

### use level

* MRF's data are defined at low level(by programer).
* Nature's data are defined at high level by businness manager.

### use case

* MRF used in OLAP
* Nature used in BP

## compare to MQ

### Nature:

Easy and union redo ability.except finst call from outer all following process will automatic retry when **Environment Error** occurs.
Even so the first call from outer which wuould be UI usually, so no need retry logic at all.

### MQ
need to handle every call for retry from outer.

## compare to RPC

RPC 拥有服务治理，熔断等特性。

https://mp.weixin.qq.com/s?__biz=MjM5MDE0Mjc4MA==&mid=2651006475&idx=3&sn=7b8da1df204e7fd250246c6554c8d363&chksm=bdbede588ac9574e2d8501b9f178e635a9aa94634f4017d52bba005dccffe89a980c05d6c465&scene=0#rd

2015 年是流计算百花齐放的时代，各个流计算框架层出不穷。Storm, JStorm, Heron, Flink, Spark Streaming, Google Dataflow (后来的 Beam) 等等。其中 Flink 的一致性语义和最接近 Dataflow 模型的开源实现，使其成为流计算框架中最耀眼的一颗。也许这也是阿里看中 Flink的原因，并决心投入重金去研究基于 Flink的 Blink框架。

如果问是基于什么具体的原因使得阿里选择了 Flink框架，阿里巴巴的高级技术专家大沙曾言，他们是在 2015年开始调研新一代流计算引擎的，当时的目标就是要设计一款低延迟、exactly once、流和批统一的，能够支撑足够大体量的复杂计算的引擎。

Spark streaming的本质还是一款基于 microbatch计算的引擎。这种引擎一个天生的缺点就是每个 microbatch的调度开销比较大。Kafka streaming是从一个日志系统做起来的，它的设计目标是足够轻量，足够简洁易用。这一点很难满足对大体量的复杂计算的需求。Storm是一个没有批处理能力的数据流处理器，除此之外 Storm只提供了非常底层的 API，用户需要自己实现很多复杂的逻辑。


