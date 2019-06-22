# Architecture

Before to read this, I suppose you have read the [concepts](concepts.md) of the Nature.

## Space-time

Nature can be divided in tow part space and time.

Space is structures is your `meta`s and `relation`s between them. they are **spatial relevance**.  You can use them to **express everything in the world**, like a photo to show the world, it's **static** also.

Time is you `instance`s, new instances will be generated along the time line. `Nature` make `instance`s flow by rules(relations),  like a train can only run on tracks, and record changes and the branches they had taken. These instances are time-dependent. like music or movies,  they are **dynamic** also. 

You can see that `meta` and `relation`s control the whole thing.  Nature abstract all kinds of things to **Space**, and can only generate one thing at runtime: `intance`, it's the soul of Nature. This abstract **decouple business logic into components**, and greatly **unify  the runtime technical logics**, such as concurrent, idempotent and retry etcetera, so you can get free from them, and focus on your business logics.



## unfinished



status data have versions to remark every change. all data history could be __traced__, and all data unchanged, even it is a status data.

she decouple business data from technology such as __service__, __Interface__ etc.,

如何实现幂等、一致性

retry

状态数据是如何实现的



只执行一次语义

Nature __short process__ can organize all the your business into a web directly,



one input and one output



## Status Data & Stateless Data

stateless data have only one version for an instance, but status data can have many version for a instance. If we look from outer, the stateless is immutable and status data is mutable.

例如 Order and OrderStatus.

## handle error

environment exception and business exception

## compatibility



## hot pluggable

# How it works

## save `Task` and `Instance` together as atomic at save time without Transaction.

Save task for store first, then save instance. and do the following tasks.

