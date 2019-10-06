# Architecture

Before to read this, I suppose you have read the [concepts](concepts.md) of the Nature.

## Space-time

Nature can be divided in tow part space and time.

Space is structures,  it is your `meta`s and `relation`s between them. they are **spatial relevance**.  You can use them to **express everything in the world**, like a photo to show the world, but it's **static** also. 

Time is your `instance`s, new instances will be generated along the time line. `Nature` make `instance`s flow by rules(relations),  like a train can only run on tracks, and record changes and the branches they had taken. These instances are time-dependent. like music or movies,  they are **dynamic** also. 

You can see that `meta` and `relation`s control the whole thing.  Nature abstract all kinds of things to space, and can only generate one thing at runtime: `intance`, That is the Nature' all. Tt like DNA and proteins in biology, `meta`-and-`relation`  are DNA which control the generation of proteins, and `instance` is the protein. This abstract **decouple business logic into components**, and greatly **unify  the runtime technical logics**, such as concurrent, idempotent and retry etcetera, so you can get free from them, and focus on your business logics wholly.

Behand `space-time` there are tow theories

- one for **science**: y=f(x)
- one for **philosophy**: choose my onw destiny

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



