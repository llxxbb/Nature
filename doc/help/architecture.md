# Architecture

Before to read this, I suppose you have known the [concepts](concepts.md) of the Nature.



![Alt text](G:/lxb/rust/Nature/doc/img/%E5%B9%BB%E7%81%AF%E7%89%878.jpg)

I named it here `Thing`.
It let you to shrink it's range and make it easy to study, this is why I call it `Thing` but not layer.
A `Thing` may have many points and `Relation` may exits between points.
The `Thing` and point are __Static__, once they are created they can't be changed.

Relative to `Thing` and point, `Relation` is __Instantaneous__, either it happened or it has not.
so relationships are very time-dependent，`Relation`'s can change over time.
Maybe __Event__ is a good word to express the meaning. but event has a random meaning,
yet relationships emphasize specific behaviors of specific participants,
so I choice `Relation`

![Alt text](G:/lxb/rust/Nature/doc/img/%E5%B9%BB%E7%81%AF%E7%89%879.jpg)
Let me say more about `Thing` and point. 
A `Thing`may have points, and points maybe another `Thing` for sublayer points.
So point and `Thing` are conceptually equivalent.
I think `Thing` feel more meaningful than point. so I use `Thing` to express `Thing` and or point.

An amazing thing is that `Thing` contains __Hierarchy__. 
The benefit of this is simplify Nature and give freedom management ability to human. 
`Nature` don't care of __Hierarchy__ but need to identify every `Thing`, 
this is a easy way to satisfy `Nature` and human management simultaneously.

![Alt text](G:/lxb/rust/Nature/doc/img/%E5%B9%BB%E7%81%AF%E7%89%8710.jpg)
`Thing` is only a define, at run-time you must bind an `Instance` to `Thing`.
If a `Relation` exists between two `Thing`'s, 
`Nature` will give upstream `Instasnce` to a converter which is binding to the certain `Relation` and it is at anywhere outside the `Nature`.
Then the converter generate new `Instance`s for the downstream `Thing`.
    
Here the `Relation` made a great job: It locked the __Geographic and Time Coordinates__, 
they are important for `Natrue` to retry when failed the execution. 

![Alt text](G:/lxb/rust/Nature/doc/img/%E5%B9%BB%E7%81%AF%E7%89%8711.jpg)

Converter is just one simple function, and the converter achieve __only one__ goal for the `Nature`.
Converter do no need to know all the other goals in `Nature`,
`Nature` will take care of them by call the converters recursively.

![Alt text](G:/lxb/rust/Nature/doc/img/%E5%B9%BB%E7%81%AF%E7%89%8712.jpg)
It is very easy to organize the goals in `Nature`.
There is no __Flow Control__, so there is no  __loop__, no __if__ to use to do __Business Process orchestration__ in `Nature`.

down-thing to __select__ upper-thing but not upper-thing to __control__ down-things.
because direct upper-things are more less down-things and __control__ will more complex than __select__..
This mechanism greatly simplifies the process.

Another key point is `Instance` bind to `Relation` but not to a __technical__ `Interface`.
It's purl business dependence,it has nothing to do with technology.
Then `Relation` made a __very short process__, only one step,
but the __simple relation__ can form any complex and large network.

There are good news for developers too, __short process__ will be easy to develop, 
and `Nature` try to run the __short process__ in parallel.

![Alt text](G:/lxb/rust/Nature/doc/img/%E5%B9%BB%E7%81%AF%E7%89%8713.jpg)

`Nature` only insert `Instance` to db, no __deletion__ no __update__.
`Nature` use __state version__ to express any chang on an `instance` and not all `Thing` have states.
This feature let `Nature` run converter equally for any times, tt is __idempotent__.





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

