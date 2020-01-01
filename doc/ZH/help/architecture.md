# Nature 架构

在阅读之前请确保对Nature 的[概念](../../../README.md)有所了解。

## 时空

Nature 是一种简单的架构，她用 `时空` 关系来简化对复杂世界的描述。`时空`将整个业务系统分成`设计时`和`运行时`，**实现了业务需要描述和功能实现的解耦**。**并建立起`设计时`对`运行时`的强制约束机制**，使代码少走弯路。

`空间`就是Nature的`设计时`，Nature 用 `Meta` 来构造点， 用 `Relation` 来构造边，将所有的点和边链接在一起就可以构成无限延展的`空间`，有了`空间`就有了结构。点代表了业务对象，边代表了业务对象之间的关系，而结构则代表了业务布局。

`时间`就是`运行时`，`时间`由 Nature 的 `Instance` 来体现，是某个时刻的业务数据表示。有时间顺序的一连串`Instance`就构成了功能，**`Instance` 一旦生成将不可变更，成为不可篡改的历史**。

## 时空关系

`Meta` 用于指导在`运行时`生成什么样的`instance`。`Relation` 用于说明在`运行时`应提供什么样的原材料来生成`Instance`。

`Relation `**提供了`设计时`对`运行时`的完全支配能力。**她将传统开发方式下系统功能间的联系强行斩断，使得彼此间关系松散，更容易开发和维护。功能必须依托与结构才能发挥作用，既`时间`依附与`空间`才能存在。既`Instance`的生成必须遵循`Relation`,的规则，就像运动必须遵循物理定律。所以**Nature 的`空间`是整个业务系统的指挥平台**。

Nature 提供了对`结构`的`动态调整`能力。对`Mata`引入了变更版本化控制技术，这使得`空间`和`时间`两个维度拥有各自独立发展的自由度，既不会对既有空间造成影响，也不会对既有`运行时`造成影响，保持了现有功能的稳定，**杜绝了传统开发方式中既有代码对迭代的制约作用**。

## y=f(x)是关系而不是功能

`Relation`可以用y=f(x)来表示。`Relation`在功能上你可以看做是传统意义上的接口。接口是一个非常重要概念，是功能间衔接的桥梁，在当下系统中具有举足轻重的地位。接口设计的合理性与否，直接决定了系统的扩展能力。但在Nature中`Relation`将接口的重要性弱化了，这反应在以下几个方面：

#### 不需要名字

这对于接口来讲是难以想象的。接口的名字是对功能的一种诠释，是**一种功能导向的产物**。言外之意就是，我必须通过这个功能才能实现目标。而Nature 不是面向功能的，而是面向目标的。对用户来讲真正重要的不是功能，而是功能所实现的目标。`Relation` 只是说要实现B目标，则需要`Meta` A 的数据， 至于使用哪种功能来实现这个目标并不在意。所以功能的名字也就没有什么存在的意义了，这就为目标和功能分离提供了理论支撑。

也就是说 Nature 将目标和功能的在系统中的价值体现给掉了个个，**将原本本末倒置的从属关系进行了正位。使其有了“自然”的表达**。借助`Relation` **Nature 把功能放到了幕后**，功能不再重要，这为功能的替换提供了便利；同时Nature又把目标间的相关性推到了前台，这是一种极简又直白的表达方式。这样**业务需求与技术实现就有了直接的桥梁**，减少了沟通的环节，使管理更加高效，不像传统开发方式那样将这种联系含蓄的隐晦的藏在功能下面，成为不好管理的**黑箱**；同时也避免了既有系统拖业务变更后腿的现象发生。

#### 只有一个入参和一个出参

从技术上讲，任何数据都可以用JSON来表示，所以`Relation`只需要一个入参就可以了。之所有有这样的要求，是为了有一个简单的模型来表示目标之间的关系：一对一，我们可以用多个一对一来表示一对多、多对多和多对一等多种复杂的情形。也就是说Nature 只需要维护一堆一对一的`Relation`就可以表达任意复杂的业务模型，而一对多，多对一等复杂情况会自然地在Nature中“涌现”出来，这就大大简化了Nature 本身的复杂度。

因为形式的简单统一，使得我们可以很容易赋予 Nature 很多增值功能，如并发、幂等、重试等，这大幅度的简化开发人员的负担，使开发者能够更好的将精力聚焦到业务本身上，同时因为开发复杂性降低，间接提高了系统的可维护性和健壮性。

## 选择自己的命运

上一小节主要从技术层面描述`Relation`，这一小节我们从哲学角度来分析`Relation`。关系是业务系统中最重要也是最复杂的部分，关系越复杂系统也就越复杂。这里我们重点说一说"一对多"和"一对一"。

举一个老板和员工的关系



这是个哲学理论

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



