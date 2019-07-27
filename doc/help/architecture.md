# Architecture

Before to read this, I suppose you have read the [concepts](concepts.md) of the Nature.

## Space-time

Nature can be divided in tow part space and time.

Space is structures is your `meta`s and `relation`s between them. they are **spatial relevance**.  You can use them to **express everything in the world**, like a photo to show the world, it's **static** also. 

Time is your `instance`s, new instances will be generated along the time line. `Nature` make `instance`s flow by rules(relations),  like a train can only run on tracks, and record changes and the branches they had taken. These instances are time-dependent. like music or movies,  they are **dynamic** also. 

You can see that `meta` and `relation`s control the whole thing.  Nature abstract all kinds of things to **Space**, and can only generate one thing at runtime: `intance`, it's the soul of Nature. This like DNA and proteins in biology, `meta`-and-`relation`  is DNA to control the protein generation, and `instance` is the protein. This abstract **decouple business logic into components**, and greatly **unify  the runtime technical logics**, such as concurrent, idempotent and retry etcetera, so you can get free from them, and focus on your business logics.

Behand `space-time` there are tow theories

- one for **science**: y=f(x)
- one for **philosophy**: choose my onw destiny

## y=f(x)

Developers use functions to describe the complex world in the computer program field. There are great different between functions,  **a great diversity of** input-parameter, output-parameter and logic body, so it's very hard to read someone's code, so there are many "bad" history project running now yet. Though there are "good" specifications to constrain development, but the diversity is the soul for a language. 

Function's free style is the main cause of the problem.  because most of the results of the functions are middle-results, this cause huge workload put on to the process-management, but they are exactly not important for user, this is determined by function's nature property. Nature focus on goals but not process, Nature break the process of a normal function into pieces(little goals), and make these pieces easy to implement: some simplified functions, Nature call these `converter`s. 

`converter` only receive one input-parameter and one output-parameter, and `converter` is a property of a `relation`. You see, Nature give a great limit to function's style, and more, **Nature try to hide function to be seen** too. That will make it easy to management, because a long process will be divide into many `converter` to implement, the **black-box** of manage than will be broken too, so this can reduce the cost of the process management. but how does it work?

All diversity of input-parameter and output-parameter call be expressed to a `JSON` object, so Nature unified the form of the functions, and all `converter`s's style are **y=f(x)**,  a linear equation with one variable, that is to say function can not to define input-parameter and output-parameter self. 

Nature care about the **x** and **y** only but not the `converter`, this unify separate data from functions, that is to say  developer can not determine the data but manager can, and functions can be easy replaced. This may rise the efficiency of management and easy the function development. So this unify is of great significance, because it can let you to choose your own destiny.

## Choose My Own Destiny

The `relation` between data is important,  but the more relationships, the more complicated. For example, relationships between boss and employees, from the boss end we can see that he have many employees, it's **one-to-many**, it's complicated; but from the employee end there is one relationship connected, it's **one-to-one**, it's simple. Nature maybe can not reduce the relations, but Nature let you have one-to-one relation only.

Thank to the unify of **y=f(x)**, Nature can make pure **data-flow**, and this make it easy to organize the business logic. The downstream know what upstream he wanted, so he can **select** a **x** as his input and don't care about how many downstream after him. so there is no **control-flow** in `relation`.  no such **branch, loop** complicated will be seen in Nature, and this simplify the develop process greatly, because control means to-many, when one of the downstream changed, the upstream might need to be modify.  but **select** only affects itself.

Though you can't see control-flow in Nature, but the control-flow just in there. Same upstream different downstream will make branch; different upstream same downstream will make confluence. All control logic are formed naturally, that is to say control-flow is not designed by you but it **spring up** itself. Furthermore, `relation` is the **one-step** of the data-flow.  all `relation`s can connected together to form a large business web and you can modify the web anywhere freely and easily, I think this is difficulty for **hard control logic** used by normal business system implement. 

## unfinished

 One-Step



## 

## Eliminate Uncertainty 

消除下一步操作中的“不确定的数据”，如，提前确定下一步要用的 `task.id`，这样在重新执行任务时就不会产生多余的副本。

消除提交数据的不确定性。









## 

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

