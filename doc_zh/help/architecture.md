# Nature 架构

在阅读之前请确保对Nature 的[概念](../../README.md)有所了解。

## 时空

Nature 是一种简单的架构，她用 `时空` 关系来简化对复杂世界的描述。`时空`将整个业务系统分成`设计时`和`运行时`，**实现了业务需要描述和功能实现的解耦**。**并建立起`设计时`对`运行时`的强制约束机制**，使代码少走弯路。

### 空间

`空间`就是Nature的`设计时`，Nature 用 `Meta` 来构造点， 用 `Relation` 来构造边，将所有的点和边链接在一起就可以构成无限延展的`空间`，有了`空间`就有了结构。点代表了业务对象，边代表了业务对象之间的关系，而结构则代表了业务布局。

**`Relation `使得`设计时`对`运行时`具有完全的支配能力**，她切断了功能间的直接联系，使得功能间更松散，更容易开发和维护。

Nature 提供了对这个`结构`的`动态调整`能力。引入了变更版本化控制技术，**避免了传统开发方式中既有代码对迭代制约作用**。



### 时间

`时间`由 Nature 的 `Instance` 来体现，是某个时刻的业务数据表示。在时间长河里不断会有各种业务对象的 `Instance` 生成，这些`Instance`将成为**不可变更的历史**。

`时间`就是`运行时`，是功能，功能是一系列的动作过程的集合，功能必须依托与结构才能发挥作用，既`时间`依附与`空间`才能存在。`运行时`完全遵循`设计时`的规则来运转。

### 时空关系





并使`运行时`按规则运转以生成业务数据：`Instance`，所以Nature 的`空间`是整个业务系统的指挥平台。

Nature 用版本记录`空间`的变化，不会对既有空间造成影响，也就不会对既有`运行时`造成影响。

`Meta` 用于指导在`运行时`生成什么样的`instance`。

`Relation` 用于说明在`运行时`应提供什么样的原材料来生成`Instance`，`Instance`的生成必须遵循`Relation`,的规则，就像运动必须遵循物理定律。

## 简化的运行时

由于`Relation`使用统一且简单的形式，这就赋予了 Nature 对`运行很容易做**切面（AOP）** ，这可以大幅度的简化开发人员的技术性工作，如并发、幂等、重试等，使开发者能够更好的将精力聚焦到业务本身上。因为开发复杂性降低，也提高了系统的可维护性和健壮性。



Behand `space-time` there are tow theories

- one for **science**: y=f(x)
- one for **philosophy**: choose my onw destiny

Nature 在使用层面保证了`时空`关系解耦，并确立了`空间`对`时间`的决定和主导作用。并使两个维度拥有独立发展的自由度，同时又保持功能依赖的可靠性。杜绝传统方式中代码对结构的制约作用.



##### 为什么用时空这么大的概念？





- 

### y=f(x)

Developers use functions to describe the complex world in the computer program field. There are great different between functions,  **a great diversity of** input-parameter, output-parameter and logic body, so it's very hard to read someone's code, so there are many "bad" history project running now yet. Though there are "good" specifications to constrain development, but the diversity is the soul for a language. 

Function's free style is the main cause of the problem.  because most of the results of the functions are middle-results, this cause huge workload put on to the process-management, but they are exactly not important for user, this is determined by function's nature property. Nature focus on goals but not process, Nature break the process of a normal function into pieces(little goals), and make these pieces easy to implement: some simplified functions, Nature call these `converter`s. 

`converter` only receive one input-parameter and one output-parameter, and `converter` is a property of a `relation`. You see, Nature give a great limit to function's style, and more, **Nature try to hide function to be seen** too. That will make it easy to management, because a long process will be divide into many `converter` to implement, the **black-box** of manage than will be broken too, so this can reduce the cost of the process management. but how does it work?

All diversity of input-parameter and output-parameter call be expressed to a `JSON` object, so Nature unified the form of the functions, and all `converter`s's style are **y=f(x)**,  a linear equation with one variable, that is to say function can not to define input-parameter and output-parameter self. 

Nature care about the **x** and **y** only but not the `converter`, this unify separate data from functions, that is to say  developer can not determine the data but manager can, and functions can be easy replaced. This may rise the efficiency of management and easy the function development. So this unify is of great significance, because it can let you to choose your own destiny.

### Choose My Own Destiny

The `relation` between data is important,  but the more relationships, the more complicated. For example, relationships between boss and employees, from the boss end we can see that he have many employees, it's **one-to-many**, it's complicated; but from the employee end there is one relationship connected, it's **one-to-one**, it's simple. Nature maybe can not reduce the relations, but Nature let you have one-to-one relation only.

Thank to the unify of **y=f(x)**, Nature can make pure **data-flow**, and this make it easy to organize the business logic. The downstream know what upstream he wanted, so he can **select** a **x** as his input and don't care about how many downstream after him. so there is no **control-flow** in `relation`.  no such **branch, loop** complicated will be seen in Nature but Nature do it for you at backend, and this simplify the develop process greatly, because control means to-many, when one of the downstream changed, the upstream might need to be modified.  but **select** only affects itself.

Furthermore, `relation` is the **one-step** of the data-flow.  all `relation`s can connected together to form a large business web and you can modify the web anywhere freely and easily, this is difficulty for **hard control logic** for normal business system implement. 

Though you can't see control-flow in Nature, but the control-flow just in there. Same upstream different downstream will make branch; different upstream same downstream will make confluence. All control logic are formed naturally, that is to say control-flow is not designed by you but it **spring up** itself.

### Data driven vs. function driven

the unify of **y=f(x)** not just hide the **control-flow** but also the functions. On the business side this is a great important thing, it reduce the complexity significantly, you just think about what are you want. 

Of cause, it's not that easy, your must think of middle-data along the way, but it is much more easy then function. In this model you use some data to compose another data until the goal can be achieved. The manager can modify a big business system plan without interference by data only. but when he face to functions, there are all kind of problems emerged, why? because function coupling too many things: language, framework, developer capabilities, deploy environment and other things, they are all dynamic and complex to manage! 

**Data driven can give you a clear,simple and good view, but function driven make you confusing**.

## Consistency

Though the **control-flow**  spring up itself, Nature give a deep control under your choice, such as dispatch tasks, retry tasks and store `instance`s generated, includes `instance`s inputted from out of Nature. It is hard to make data consistent in a complex network environment in normal business system, but Nature encapsulate those complexity for you. 

### Idempotent

Idempotent is important and obligatory when retry exists, `Nature` only insert data to database, no __deletions__ no __updates__. Once they inputted, you can't change any of them, even for the state data.

Nature is **making history**.

There are some cases for retries

- `instance` inputted from outside
- dispatch tasks to `converter`s
- `instance` converted by `converter`

#### Save task before data

Let's to see the dispatch-task first, there is an example : One upstream has tow downstream flows,  and Nature failed for the  first downstream generating and succeed for the second downstream; and at that time we do a dangerous operation that we removed the first downstream `relation` definition from the database; and then the Nature retry the the failed for the first branch. Boom! same input get different outputs, So Nature must to avoid this case happen. One possible way to do this is **generate all tasks before dispatch**, so that the `relation` changes will not take affect on the retrying tasks. 

But the task maybe be created many times on bad environment. When instance is same, nature will delete the new task to avoid unnecessary processing.

But there is another problem: save 'instance' and generate converter tasks may be broken on bad network environment.  You may say database **transaction** can resolve it,  considering the large distribute database system will be used, so **Nature can not use the database-transaction**.  To resolve this problem, **Nature will save task before save instances**, so that Nature retry can rebuild all data consistently.

#### Save plan before data

Now for the third case. a `converter` may return many instances,  because we can not use transaction,  all these need to be saved one by one,  It can be interrupt by bad environment also. Nature introduced `plan` to resolve it. Plan is a big object include all returned instances. **before  we save `instance`s for each, we save `plan` first**, so that we can redo it when instances saving is broken. 

But there is a particular case be ignored, the `converter` may be not idempotent, that mean the `plan` may be changed. Nature does not allow this happen: the `plan` table's primary key is made up of upstream `meta` and downstream `meta`,  in this mechanism Nature would reject all the later plans that with the same upstream and downstream, **the first is the last**.

#### Primary key of the `Instance` table

Another point is instance table. same as `plan` described upper, Nature only insert data to it too, and the table's primary key is little complex, it is made up of id, `meta` and state version. But in fact this is not enough,  id is the stumbling  block when instance inputted from outside. Id must be unique, if you don't give one to Nature, Nature generated one by hash. so it's idempotent in this situation.  Theoretically, hash algorithm has conflict problem, though it's a small chance, so Nature recommends to use your own unique id. Maybe a center-id-generator like facebook 's snowflake is a good choice..

### Error, retry and callback

For `converter` Nature defined two type of error:

- `ConverterLogicalError`
- `ConverterEnvironmentError`

If the `converter`  encounters an undefined condition and should break the process, it can return a `ConverterLogicalError` then Nature will move the task from task-table to task-error-table and don't retry it anymore. 

The `ConverterEnvironmentError` will be caught by Nature itself for network error. Nature implemented a strong retry mechanism to retry the failed task for many times, if all that retry are failed, the task will move to task-error-table too.  In there user can find the error tasks and get know what error happened to the task.

There is a special  `ConverterEnvironmentError`: timeout. If  the `converter` will spend much time to process, then every retry will cause timeout. In that case Nature provide `callback` mechanism to resolve it. When Nature call the `converter`, converter can return a asynchronized signal with a time to be deferred instead of instances immediately, then Nature will suspend the task. Within the deferred time,  `converter` can process the task in another thread. When finished, `converter` then call Nature's `callback`  interface, then the suspended task will go on. But if no `callback` occurred, Nature will do the retry.

## hot pluggable

Nature is a platform focus on business and simplify it, that loose couples technology and business. So Nature make technology more generic and easy to integrate. such monitor, authorize and visualization etcetera.



