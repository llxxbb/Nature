# Nature

Nature 是一种新的构建大型系统的方式，她会让你“**失去对下游节点的控制”**，用下游**“自然选择”**上游的开放模式来构建复杂的系统。因为控制和依赖的减少业务间的耦合度将变得非常低，进而业务系统本身的**“进化”**将变得前所未有的简单和高效，这会大量减少系统的管理、建设和维护成本。这也是本项目名称的由来。

数据是系统的灵魂，是企业的灵魂，但现状是必须借助研发体系来驾驭，所以效率沟通很成问题。而 Nature 让懂业务的人描述数据、掌控数据流向，从而使开发人员失去对数据控制的主导地位。不用去担心算法问题，Nature 已内嵌常规的算法。

驱动数据有效工作的常规设计已经被Nature。

Nature free data define from program processing. Nature isolate **Business-Goals** from execution clearly, that will make you system adjust quickly , more correctly and easy to maintain. Unlike a `Service`, which bind goals and implements together, commonly, **the service would make data dirty and hardly to maintains**. 

If you are very understanding of the business but you are not a programmer, by Nature you will be a good leader for the business system.

Nature Help you to manage your data from a __High Perspective__.  All your **Business-Domain** can be viewed and be managed globally; and more, all the **relations** between domain can be choreographed globally also. Just like a terrestrial globe, all things on it. 

Nature is `FaaS`(Function as a Service) platform technically. But I don't like to use this concept, because I don't want to hype a concept or a fan of it, `FaaS` is not the primary goal for this project. Nonetheless `FaaS` is the key point to make Nature working. Nature break meta-relations into pieces(a function) for developer to implement, and then put them together at runtime.

__Important__:  this project is in a very earlier stage, it is far to mature.

## A big question

This project covered a lot of  fields (__Work-Flow__, __Message Queue__, __Stream__, __BPMN__, __Gateway__, __Distributed_System__ and __Database__) , and each field have their own mature and good projects. why do I provide a new one? 

I have no intention of competing with other fields.  it's just involves these field. Sorry for the  __Nature__ name for this project.  Nature means all things,  surely this it can not do that. But I could not think out a suitable name for it. 

Please do not think it too complicated. there are only few things be introduced in this project: __data__ and __relations__ between data. Just like atoms and there relations make up the complex world, in this project, __data__ make up all your business. So I like the name __Nature__. let us to see it deeper.

### Functions vs goals

Work-Flow and BPMN are __function__ oriented solutions, it may let business run, but when business growing too large, you may __lost in functions__, they call tell you how-to-do, but do-for-what is uneasy  to describe. because function can __hide goals(data) inadvertently__, then the business system is hard to understand, and you will find it is difficult to find out what your really want.

Message-Queue and Stream face to connection more than data: a connector between functions. Nevertheless I think they are better then Work-Flow and BPMN, because they rise the data's value. But I do not think it is enough yet, it let developers are both a player and a referee. 

Nature make data(__goals__) clearly and isolate data from functions and hide functions back-end. **data are pure energy** for system running. , so you can get a agiler and powerful business system based on Nature,.

### Bottom-up management vs top-down management

__Gateway__, __Distributed_System__ and __Database__ are __bottom-up management__. They are technology oriented, there manage their own affairs in their own way. Because they are different things, so it's hard to unify and simplify the management.

Nature let you declare **business-goals** (not technical things) and **relation** between them and **who** can do transfer between the goals(execute the goals), this is all you need to manage, Nature and developers will take care **How** to execute the goals. This is a **top-down** manner of management, it's simple, directly and effectively.  From the **top** we can glue multiple elements together and make it 1 + 1 > 2.

- Nature is a __gateway__: Nature can control and route all you business request.
- Nature is a __distributed system__: you needn't care about data availability and consistently.
- Nature is a __KV-database__:  you can query data you putted in and  Nature itself transferred, and your data will __never be lost and changed__ when they are written down.

## What's in it for you

Let to see what benefit to you when using Nature

- give a clearly interface between **what** and **how**.  and achieve the goal  **no distortion**, **full restraint**. this will reduce communication costs greatly in your team. 
- Easy to focus on goals and less detours, you will __never lost your target__ among a big system..

- Development task is more easier and Nature can __speed up your development iterations__: 
  - All workpiece is pluggable. developer focus one goals once a time, need not to have a global view and understand the whole thing, though it is easy to present by Nature.
  - Need not to take care about data consistently, high concurrency, idempotent and other things.  so you can reduce the need of *senior programmer*s and then reduce the cost.
- More directly, Nature will save you time and money,  __bigger is cheaper__.

## Usage scenarios

Complex business system like web store.

## Want to know more?

[A concrete example](https://github.com/llxxbb/Nature-Demo)

[Concepts](doc\help\concepts.md)

[Architecture](doc\help\architecture.md)

[Reference](doc\help\reference.md)





# Nature

The Virtual World is made of data, VW is made of connection also. Data is static and present at certain time, connection is dynamic and along the time make date to connect each other. So the tow important abstraction made the VM running.

This project allow you define you data and connections between data. connection can be processed by Converter defined by you. Those are all you need, then a distribute, effective, consistent, fault tolerance system is ready for you.

## 大数据处理我们只需要一个解决方案就可以了

消息中间间, 流式计算, spring cloud 各自只解决一部分问题。事件源。

### 可扩展性

* 分布式服务
    每个节点都是职能对等的，可通过增加服务器来达到线性扩展。
* 分布式数据库

### 可用性

* 数据一致性
* 幂等
* 重试

### 可靠性

* 重试
