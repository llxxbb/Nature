# Nature Architecture Thought

Before reading, please read [README](../../../README_EN.md) to understand some concepts and terminology.

## Nature's view of nature 

Nature was proposed to **simplify the construction of business systems**, which must be simple to use and capable of carrying complex services. This is actually quite difficult. For this reason, Nature draws on the wisdom of the ancient Chinese "Book of Changes". It is said in "Yi Zhuan": "Change has Tai Chi, which gives birth to two instruments, two instruments give birth to four images, and four images give birth to gossip." , which just meets the core demands of Nature. 

The reason why this system is named Nature is to hope that Nature can use simple rules to deal with a wide range of complex business problems.

### Problems with traditional systems

#### Single system problem 

Under normal circumstances, we need to go through many steps to transform our ideas into code, such as: requirement research, design, coding, testing, deployment, etc. Only in this way can the code "really understand" our requirements. Nature calls this **decision solidification **, which means that the system is **a mixture of decision and execution**. Therefore, **the research and development cycle is relatively long**, and it is often iterated, which consumes more resources. 

Sadly, the mixture is mixed with too many technical elements, so that decision makers cannot clearly understand whether the decision is executed perfectly. Only technical personnel know what they really did. This is an **inherent problem** in traditional development methods, and the root of the problem lies in **customization**. Standardization performs very well in the purely technical field, and various open source components can illustrate this very well. But for the business field, in the era of homogeneous competition, each service provider competes with its opponents in details. These details will inevitably lead to customization, so basically there is no business system that can be reused.

#### Multi-body system problems

When the business get more complex, we generally use multiple systems to carry the business. Most of the systems are divided according to the **vertical fields** of the business. Take e-commerce as an example. They can be roughly divided into transaction, warehouse, distribution, after-sales, financial, and user systems. Most of such parts maintain a **mapping relationship** with the division of organizations, which is intuitive and is also common knowledge in the industry. However, the larger the system scale, the higher the cost of collaboration between systems. This is caused by the **linkage** of decision-making, especially on the core link, the iteration of the system will become **extremely difficult**.

The root cause of the problem is still the solidification of the decision. The change of the decision is inevitably more or less constrained by the existing system. This is manifested in the fact that the relevant system **cannot be deployed at the same time directly**, so we need to consider the **smooth** and **compatibility** way, which form some transitional  code that is **difficult to remove** (programmers are **good at addition** but not good at subtraction), which will undoubtedly increase the system complexity greatly. After a long period of iteration, the system will become **bloated** and **difficult to maintain**, so it is not surprising that a development team has hundreds or thousands of people. Although we can avoid some problems through good design, but it is difficult to predict which is the good design for the development of the business especially it is rapid at initial, and it is difficult for us to have a perfect design  at the beginning when we face a fleeting business opportunity, so this is not realistic.

### way of solving the problem

To solve this problem, we need to solve the problem of customization first. So, Nature provides [Meta](meta.md) and [Relation](relation.md) to standardize the decision, The definitions of `Meta` and `Relation` do not need to be coded, it is data that can be recognized by the machine, otherwise the decision is another form of solidification. Decision can be freed from the code by the standardization, but the freed decision information needs a new carrier and don't rely on the business system, this is Nature.

The most important thing for decision-making is the goal we want to achieve. The goal is data, because the output of the system is data. The data here is a general reference, so Nature calls it `Meta`. Once we have goals, we need to define methods and rules to accomplish them. Methods and rules must establish `Relation`s between multiple data, because they need to be inputted and outputted, otherwise the rules and methods will lose their existence significance. The business must define `Meta` and `Relation` before running, so `Meta` and `Relation` are also called design time.

With `Meta` and `Relation`, we can take out all the decisions from the traditional business system and put them together, and put the rest together too, so we can get two independent sets. If Nature is Tai Chi, then these two sets are Liang Yi. In this way, decentralized decisions can form into an organism, and **they would be centralized**. This is especially useful for multi-body systems, without considering the various problems caused by mutual entanglement. After the decision is taken out of the business system, the original business system will lose its soul! This will **cause the complete failure of traditional development methods**!

When the rules and methods are set, someone must come to play. The remaining parts that lose their souls are these players. Nature calls it [Executor](executor.md). The essence of `Executor` is to complete the conversion of associated data in `Relation`. In most cases, `Executor` must be implemented by you. Please don't worry about, the implementation is very simple and has single responsibilities, because the complex business coupling has been undertaken by `Relation`. `Executor` is also called runtime.

Nature's `design time` has full control over the `run time`. `Meta` is used to limit what kind of `instance` is generated at runtime. And `Instance` is specific data generated based on `Meta` at a specific time and in a specific scenario. The relationship between `Meta` and `Instance` is equivalent to `Class` and `Object` in programming languages. `Relation` specifies the rules of `Runtime`, which means that Nature's **`Meta` and `Relation` are the command center of the entire business system**.

`Meta`, `Relation`, `Executor` and `Instance` are abstract, concise, standardized and unified forms, so they can carry many different business demands. Does this look like the four images in the "Book of Changes"?

## Nature's Time and Space View

The above is mainly a problem-oriented way to illustrate how Nature solves the problem. Next, we will explain Nature's design philosophy.

Decision provides execution space and rules for space conversion. Space needs supporters, For a geometric body,  the supporters are points, lines, and surfaces; note that "surface" is **implicit**. After the points and lines are drawn, the surface naturally comes out, and when the surface comes out the body will follow. For Nature, `Meta` is a point, representing business objects; `Relation` is a line, representing the relationship between business objects. Note: Nature "clearly" defines the "surface", which is **implied** in the hierarchical structure of `Meta#key`, which is used to represent the business field. All fields constitute a complete business system. In this way, Nature only needs  `Meta` and `Relation` the two elements to describe the complex business entities and the relationships between entities. can't be less, too much unnecessary, This is Nature's view of space.

Let’s talk about Nature’s view of time. Time represents the running, order, unalterable history and Evolution:

- Running: Only the structure (space) that is in motion can perform its functions, and the `Relation` is the engine in motion! Because Relation is standard, the mechanism used to ensure running can be built into Nature, such as idempotence, retry, exception handling, etc. This means that **most ordinary programmers can make a reliable and stable system**!
- Order: Order must be defined centrally, and then everyone must follow it. `Relation` defines the business rules that `Executor` must execute, the most important thing is that the upstream and downstream `Meta`. In this way, the order will be created, which means that `Executor` can only do its own thing, which greatly simplifies its logic.
- Unalterable history: Every change in decision will be recorded with a version, which not only expresses respect for history, but also enables unfinished business to continue. `Instance` itself represents history, **once it is generated, it will never be tampered with**, even if it is state data, every change can be **backtracked** (also based on the version number).
- Evolution: Nature breaks the decision solidification, so that **decision-making and execution can evolve independently**. Not only that, Nature makes the decision itself more concise, more precise, and more standardized; it makes the execution responsibility more single and lighter. These will make the evolution of the business easier.

## Mathematical expression of Nature's operating mechanism

Nature's operating mechanism can be expressed by a mathematical formula:

```
  y=f(f(f(...f(x)...)))
```

The meaning is: each `Relation` can be expressed as y=f(x), and the output of upstream `Relation` can be used as the input of downstream `Relation`. Where "x" and "y" are both `Meta`, and "f" is `Executor`. `Executor` can be regarded as an interface in the traditional sense. The interface plays a pivotal role in traditional development methods and it is a bridge between functions. `Relation` weakens the importance of the interface, which can have several advantages:

### Defunctionalization

The traditional interface is a kind of **function-oriented** product, function is used to achieve the goal, so **function is phenomenon-level**, you have to analyze it to know what it is going to do, and sometimes you don’t know it even, what is it doing, or why. Nature is **goal-oriented**, the goal is a point and the function is a line, so the goal is much simpler and clearer.

For a "y=f(x)", the target is "y", and "f" as an Executor is just a function placeholder, so we can simplify it to

```
x -> y
```

It is `Relation`, in this way, Nature **corrects the subordination relationship between the  function and the goal** for the traditional system, giving it a "**natural**" expression. With this expression, it will bring the following important meanings:

- We can focus more on the goal and make the goal concise, intuitive and visual.
- The goal can be standardized and get rid of the constraints of decision solidification.
- The standardization of goals drive the standardization of `Executor`.

### Decentralization

As we said above, the output of upstream `Relation` can be used as the input of downstream `Relation`. This is a method of stream processing, which can **autonomously flow** one after another and generate corresponding `Instance` at runtime. During this period, there is no need for a certain point or a few points to carry out any process control, which means that the behavior of the system is free to play by downstream, is not controlled, and is decentralized.

Decentralization is extraordinary in a large-scale system. Mainly reflected in the following aspects

- **Efficiency**: Decentralization means that the length of the control-chain  is reduced, which means that tasks can be completed in a shorter time and fewer resources would be used. It means that it can be concurrent, the capacity can be expanded horizontally, and without worrying about squeezing the single-plank bridge.
- **Adaptive**: On the premise of ensuring the completion of the task, because of the removal of control, there are more possibilities for implementation, without worrying about excessive control and constraints.

### One to One

`Relation` only allows one input to correspond to one output, which is one-to-one. This approach is borrowed from relational databases. In addition to its simple form, there are several other functions.

- **Force decoupling between businesses**. One-to-many, many-to-one and other complex relationships may be expressed in one-to-one, so that the diversity of control forms is eliminated, and it is easy to unify, which provided the foundation for processing design based on simple configuration that does not require coding.
- High-level functions are easier to achieve. Each one-to-one is simple, so we can quickly build a lot of functional bodies, when they are aggregated together through Nature, just like the bee colony and ant colony described in "Out of Control", "**emerge** " a high-level ability.
- One-to-one can make it easier for Nature to **Empower**. Because the form of `Executor` is simple and unified, it is easy for us to apply `aspect` techniques on `Executor`, such as concurrency, idempotence, retry, etc., thereby greatly reducing the technical complexity of `Executor` development, and enabling developers better focus on the business.

## Nature's Philosophical Significance of Operation Mechanism

Here we talk about the difference between **control** and **choice**: Control is the control of others, and choice is the choice of oneself and has nothing to do with others.

### Control

Control is the means and method adopted to achieve the goal. Most of our **systems are based on control**, code controls everything, whether it is a framework, design pattern, or components, it is a kind of control, including Nature, which is a matter of course since the birth of the programming industry. From this point of view, the system is a polymer of goals and functions, and the two are inseparable. **This is a clear proof of the solidification of decision**.

The more points the more difficult to control. Although we can reduce the complexity of control by layering, but it increases the length of the **feedback path**. This is a difficult decision that a large distributed system needs to face directly. The more participants, the more complex the control logic, until a **bottleneck** appears!

**Nature only controls the rules**, not the business itself, and Nature's rules are very simple and very few, which ensures the efficiency of Nature's processing. One of the most important rules in Nature is `Relation`, which refuses to control. Instead, `Relation` requires all business participants to `select`  that described below, It is **self-organize process**. From this level, Nature can greatly reduce the development costs related to business control.

### Select

Selection is the opposite of control. It is bottom-up, just like a river, the upstream cannot control the downstream flow. The form of `Relation` determines that it does not allow control of business processes, only selection. `Relation` not only selects upstream, it also selects `Executor`, so that "functions" can be replaced at will.

Because the upstream does not control, there is no need for the downstream to **feedback** the information to the upstream, it becomes a truly efficient flow, this is especially meaningful for multi-level feedback. Therefore, the selection not only improves performance, but also gives the downstream full flexibility, which is very important for business expansion and adjustment!

### Impact on business iteration

Now it is generally advocated: "Run in small steps, fast iteration, **continuous delivery**", the system bottleneck of business development can be seen in general. However, to achieve this effect is conditional, remove the peripheral dependencies such as automated deployment, DevOps, and cloud infrastructure, it depends on whether the overall structure of your projects has been stable. That is to say, this kind of continuous delivery is generally limited to small-scale adjustments, and it is difficult to achieve rapid cross-system iteration, but this is what we need most urgently! 

This is caused by the limitations of control: **Control requires controlling all participants**. When we need to change something, the control end must review and adjust all relevant participants. For an individual, in order to control it, it must **occupy** some resources in some form that it controls. The ability and scope of control are closely related to the degree of resource occupation. Occupation need to **invade**, and invasion is costly, which is very unfavorable for large distributed systems.

Here is an example of payment and delivery: when the payment status is "paid", an outbound application form is generated, and no action is taken if the payment is not completed. Usually we will use the `if` control statement in the payment logic, this is control! Now we have to make business adjustments: change the delivery dependency from the payment system to the order system. In this way, the logic of the payment system and the order system must be modified; in addition, the smooth transition of the two systems needs to be considered. This is a high-risk, time-consuming, and difficult operation with side effects. So you can see how high the cost of control is despite it is a small decision change!

Let's take a look at `Selection` again. Nature uses `Select` to **guide the flow** instead of controlling the flow, so there is no `if` in the flow control. As for the above example, the payment completion does not automatically trigger any action, but it is triggered when the downstream need it. You only need to define a `Relation` of "payment status=paid->outbound request form". Okay. When the decision becomes to order-status-driven, we only need to change `Relation` to: "Order status=paid->outbound application form" and modify the corresponding `Executor`. From this it can be seen that `selection` can achieve **modification on demand**, but which is difficult for controlling.

Nature is suitable for fast iteration, it does not have the problem of **boundary wall** between systems, which is given by the selection feature of Nature. The selection is OK as long as the other party exists and there is no need to invade, so there is no additional cost, that is, **the select is much lower environmental requirements than the control**. At the same time, because there is no control, you will not encounter complex logic problems, such as branches, loops, jumps, etc.; because there is no control, the cost of system evolution will be very low, and the links between business modules can be easily reorganized and easy to find the optimal path; because there is no control, a variety of business modules are free to test and expand, to meet the needs of business development in a flexible way.

### Ecology and Law

In an ecosystem, there is no supreme master who can control everything. Every species adapts in the selection. Only in this way can there be biological vitality, which is what the current traditional system lacks. The reason why the traditional system is lifeless and inefficient is because of top-down control, which is similar to the level of reporting and approval of the company’s business. **When the control chain is very long and wide, the internal friction caused by the control will be hard**.

Although Nature does not emphasize control, what does Nature use to maintain the order? 
That's the law. No control does not mean that there is no law, control has a clear purpose, and the law has no purpose. The reason why the earth revolves around the sun is not that the sun deliberately controls the earth, but the law of gravitation is at work. The ecosystem of nature is the same, winter and summer exchange, day and night reincarnation, the law maintains the balance of natural ecology. The law of Nature is `Selection`, that is, `Relation`. Almost all the code in Nature is organized around this rule.

### Invisible control under selection

In [Demo](https://github.com/llxxbb/Nature-Demo) there are examples related to online shopping and statistics. These examples illustrate how Nature can simplify the implementation of these services. We won’t go into details here, we just want to explain how the selection mechanism can effectively support the operation order of the system. For the sake of simplicity, the content expressed may not be exactly the same as in the Demo.

We mentioned above that the downstream select upstream, which reveals a way of thinking: **reverse thinking**, which is what we need to achieve our goals. For online shopping, it needs to be reversed from the end of the process. If the user wants to get the product, the delivery person needs to send it. The delivery data is the receipt, so we define the first `Meta`. Then we push backwards. The delivery person needs to get the goods for delivery from the warehouse. The outbound order is our second `Meta`, so we have the first `Relation`: Outbound order -> Sign receipt.

By analogy, we can define a `Relation` similar to the following

```
Outbound order -> Sign receipt
sale Order -> Outbound Order
```

There is a very important point to talk about. How to schedule from the outbound order to the receipt order, whether it is delivered by your own company or a third-party delivery, does not care here. The `Executor` of `Relation` can use whichever you want. Both `Relation` only cares about the result, not how to do it. Of course, if you have your own logistics team and want to track the logistics status, you can build a branch goal from the Outbound order. We will talk about diversion below.

Such a simple but complete `design time` came out. And this design realizes the control of `runtime` without writing code.

Outbound order -> Signed receipt is a one-to-one relationship in `design time`, and the output of `Instance` at runtime is also one-to-one. Both a real outbound order will correspond to a real sign receipt. This is the most conventional control in Nature. Other control methods are supported by this conventional method, such as the diversion described next.

The form of diversion in the design is that different downstream have the same upstream, just like a river bifurcation. For example, the above outbound order can also drive the inventory status, `Relation` is as follows:

```
Outbound order -> Sign receipt
Outbound order -> inventory status
```

In this case, the outbound order does not need to know how many downstream there are, but Nature will know, so Nature needs to execute the `Executor` of each `Relation` separately behind the scenes.

Pay attention to the "Outgoing List -> Inventory Status" here. This is one-to-one at design time, but it may be many-to-one at runtime. For example, suppose we have two outbound orders in `runtime`, outbound order A includes 2 mobile phones, and outbound order B includes 3 mobile phones, but there can only be one mobile phone inventory status, so in this example, there are two instances of outbound orders corresponding to different versions of the instance data of the same inventory status. (For status data and status concurrency control, please see [Nature Technical Features](characteristics.md))

So is there a one-to-many run-time? Here is an example: Order -> Payment Order. This is one-to-one at design time, but can be one-to-many at runtime. Assuming that the money in the user's first card is not enough to pay for the order, then there will be multiple payments, that is, one order instance corresponds to multiple payment instances.

So is there a many-to-one situation in `design time`? The answer is yes, `Relation` is defined as follows:

```
Outbound order -> inventory status
Inbound order -> inventory status
```

Now let's look at the one-to-many and many-to-one of `design time`, one-to-many and many-to-one of `runtime`, which can be combined at will, which forms a complete closed loop at the theoretical level. In this way, `Relation` can support very complex businesses. But for users, there is basically no need to care about process control issues. Users only need to make reasonable selections, and Nature will control everything under the law (`Relation`).

## Standardization

Nature breaks the traditional function-oriented development model, and its simplified form brings us the possibility of standardization while reducing costs:

One of them is the standardization of decision-making. The solidification of decision in the traditional way makes decision very individual, and only a customized system can express it. Nature's decision is standardized data, we only need to configure it, and it can be understood by the execution system without coding. However, Nature is not limited to the standardization of forms. It has a greater meaning **standardization of decision-making behavior**. Compared with the traditional way, Nature can greatly reduce the consideration of the implementation level and face the final decision product: data.

The second is **implementation standardization**. Because the coupling between services is cut off, each business execution unit is very independent, and these execution units are uniformly scheduled by Nature. This will inevitably unify the interface form and naturally standardized.

The third is **Data Standardization and Purification**. In the traditional model, we may have dozens or hundreds of databases with hundreds or thousands of data tables. These data tables are a mixture of temporary data, business definitions, business control, technical data, and business data. But Nature only has three data tables related to business: `Meta` for business definition, `Relation` for business control, and `Instance` for business data. This will greatly reduce unnecessary data storage and reduce data redundancy. Of course, we may need a large distributed database, such as [Tidb](https://pingcap.com/en/).