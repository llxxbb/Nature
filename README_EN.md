# Nature

English|[中文](README.md)

Nature is a **minimalist programming** platform. It is net-based, so it can be used in a variety of network languages. Nature simplifies your coding in the following ways:

- Nature provides a **unified data storage model and ultimate data consistency assurance**. Nature's database model can be used in many business scenarios without additional database design and code support for database logic.
- Nature provides **configuration-based process control**. Code can basically not consider process control, time scheduling, failure retry and other questions.

Nature is an innovative product that draws on messaging systems, BPMN, Workflow, FaaS and databases to integrate some of the ideas.

Nature is an open source solution for building a new class of applications based on **data-driven**, **business-oriented**, and **decentralized**. **Nature organically integrates system architecture and business architecture together**,  to bridge the gap between the two and enable the system to better serve the business.

Nature builds systems using downstream **Natural Selection** upstream instead of the traditional **upstream control downstream** approach. Changing from centrally controlled **one-to-many** to decentralized **one-to-one**, so that it can reduces transaction complexity significantly. Nature can dramatically streamline the business model and to meet the need for rapid iteration of the business, allowing the system to **evolve through constant selection**. That's what the name of the project implies.

Nature allows the business to **have complete control over the code** without being constrained in one way or another by existing systems. You can control business processes in a non-coding way, this will be important in two ways: it will make it easier to shape your system, and it will significantly reduce development investment.

## Problems with traditional development approaches

To achieve business goals, the system requires a suite of software engineering to escort it. For staffing, management and time costs, we tend to tailor to varying degrees and combine various development models, such as agile, to achieve a balance between quality and cost. But even this, research and development remains costly, because traditional development methods cannot get around the following problems.

- **The Passing Game**: Large systems generally need to be collaborated through multiple teams of different functions such as requirements, product, design, and R&D, so they often have high latency and inefficiency issues. In order to reduce these losses, we generally also introduce quality control links, which are cost reflective, the more links, the higher the cost.
- **weak controllability**: The requirements proponent is mostly not a developer, and his ideas can only be embodied on the system indirectly through others, **not directly controlled**. For the iceberg of the system, the requirements proponent is **only able to provide some guidance** for the surface portion, has no absolute control, and has no say over the underwater portion.
- **Target kidnapped**: The goal is expressed in code, yet the business wants to change and the code is hard to change! **There are always modules in the big system that people are afraid to touch! It's an eternal pain in the system.**
- **The skeletal reality**: The slogans can be shouted very clearly and loudly, but when it really gets down to the nitty-gritty and settles into the implementation details, it's hard for technicians to make the ideal system when they're in the crosshairs of business, technology, cost and other dimensions. Backed by the belief that **rapid iteration and continuous delivery** is pinned on the next correction.

- **The Lost Target**: Most of our work is functionally oriented, and we only occasionally look back at our goals. Over time, without realizing it, we get caught up in the vortex of function, and **forget what we really want**.
- **Non-functional requirements**: For a complex business system, non-functional requirements are far more difficult and time-consuming to develop than functional requirements, such as idempotent, retry, consistency, stability, reliability, etc. **These are the black holes that suck money**.

## Nature's solution

The problems faced by traditional development methods are inherent in themselves and cannot be solved by perfecting them. To solve these problems, a whole new mechanism is needed, and Nature provides such a mechanism.

Nature is business-oriented, she abstracts business at a high latitude, and all complex business worlds can be represented by **business objects** and the **relationships** between them. Business objects are actually **business goals**, and relationships are the springboard that will allow you to **achieve them one by one**. A relationship is not a method or a function, but it is a vehicle for function. It doesn't tell you what to do, but it does tell you what you need.

### Business objects have "target" attributes

On a macro level, systems are designed to process data. What you see on the monitor and what you interact with between systems is data. So **systems are made for data, and data is the goal of the system**.  Microscopically speaking, a method inputs data and outputs data. **Methods exist for processing data, and data is the goal of the method**. It follows that data is what we want, and in that sense **data is the object**. Business objects are the data that describe the business, so **business objects are the soul of the system**. If the business objects are well managed, the goals are well managed, and Nature was created to manage business objects.

### Freeing the target from the code, making it implicit to explicit

In a traditional development approach, business objects are defined by code. In Nature, business objects are called `Meta` and are defined in a configurable way. This is a humanized definition that does not require coding. The `Meta` is digital and is can directly involved in the processing of the system, that can avoid the following problems generated.

- There is **no need to transmit the target**. One person can define it just fine, and all the people see the same thing, thus avoiding the problem of distortion, so that the target delivery, translation and control costs are gone.
- The target has **strong control ability **. The identity of the code changes from definer to follower, there is no longer a constraining capital on the target, and the target has the ability to truly control rather than guiding the code. And need not worry about being kidnapped by code.
- Refinement work and business changes should just care about the business dimensions, **technical iterations will become meaningless**.

### Take away the control of the code over business processes and reduce the complexity of business systems.

Traditional business systems are complex when they grow large. Whether it's technical or business, there's a lot of logic woven together like a jumble of interlocking threads. But Nature split it into three parts: the technical part is Nature's responsibility as much as possible, and the functional implementation of Leave it to the developers and leave the most important part, process control, to the business managers, this part is configurable and can be easily controlled, without programming. This would simplify and organize a complex system.



Sorry! for unfinished translation：

剥夺代码对业务流程的控制权，降低业务系统的复杂度。



[A concrete example](https://github.com/llxxbb/Nature-Demo)

[Concepts](doc\help\concepts.md)

[Architecture](doc\help\architecture.md)

[Reference](doc\help\reference.md)

