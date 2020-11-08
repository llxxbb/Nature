# Nature Architecture

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

As we said above, the output of upstream `Relation` can be used as the input of downstream `Relation`. This is a serial streaming method, which can **autonomously flow** one after another and generate corresponding `Instance` at runtime. During this period, there is no need for a certain point or a few points to carry out any process control, which means that the behavior of the system is free to play downstream, is not controlled, and is decentralized.

Decentralization is extraordinary in a large-scale system. Mainly reflected in the following aspects

-Efficiency: Decentralization means that the control logic is reduced, which means that tasks can be completed in a shorter time and fewer resources. It means that the capacity can be expanded concurrently and horizontally without worrying about squeezing the single-plank bridge.
-Adaptive: On the premise of ensuring the completion of the task, because of the removal of control, there are more possibilities for implementation, without worrying about excessive control and constraints.