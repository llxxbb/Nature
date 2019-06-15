# Nature

Nature Help you to manage your data from a __High Perspective__.  All your **Business-Data-Metadata** can be viewed and be managed globally; and more, all the **relations** between metadata can be choreographed globally also. Just like a tellurion, all things on it. 

Nature isolate **Business-Goals** from execution clearly, that will make you system adjust quickly and make your team more **effectively**. This is the most import thing to design for this project, unlike `Service`, which bind goals and implements together. Commonly, it's hard to change when a lots of clients connected to a service in using.

Nature is `FaaS`(Function as a Service) platform technically, but I don't like to use it. I don't want to hype a concept, `FaaS` is not the primary goal for this project. Nonetheless `FaaS` is the key point to make it working. Nature break meta-relations into pieces(a function) for developer to implement, and then put them together at runtime.

Sorry for the  __Nature__ word that named for this project.  Nature means all things,  surely this it can not do that, but I could not think out a suitable word for it. It can do all the topic's (__Workflow__, __Message Queue__, __Stream__, __BPM__, __Map Reduce__, __Gateway__, __Distributed_System__ and __Database__) words which I listed for this project.

Please do not think it too complicated. there are only few things be introduced in this project: __data__ and __relation__ between data. Just like atoms and there relations make up the complex world, in this project, __data__ make up all your business. So I like the name __Nature__. 

__Important__:  this project is in a very earlier stage, it is far to mature.

## A big question

This project covered a lot of  fields, and each field have their own mature and good projects. why do I provide a new one? 

No, this project only focus on one field: __Data__. I have no intention of compelling with other fields, it's just involves these fields, let us to see it deeper.

### Functions vs goals

__Workflow__, __Message Queue__, __Stream__, __BPM__, __Map Reduce__ are __function__ oriented, it may let business go, but when business growing too large, you may __lost__ in functions, because function __code-binding__ with the goal, it can __hide goals(data) inadvertently__, then the business system is hard to understand, and you may ask why it should be like this.

Nature let you plan your __goals__ (data), it's clearly in there  for you to guide your work, because Nature __"hide functions deliberately"__, so you will get a agiler business system based on Nature.

### Bottom-up management vs top-down management

__Gateway__, __Distributed_System__ and __Database__ are __bottom-up management__. They are technology oriented, there manage their own affairs in their own way. Because they are different things, so it's hard to unify and simplify the management.

Nature let you declare **business-goals** (not technical things) and **relation** between them and **who** can do transfer between the goals(execute the goals), this is all you need to manage, Nature and developers will take care **How** to execute.

- Nature like a __gateway__: Nature will manage routing between goals.
- Nature like a __distributed system__: you needn't care about data availability and consistently.
- Nature like a __KV-database__:  you can query data you putted in and  Nature itself transferred, and your data will __never be lost and changed__ when they are written down.

## What's in it for you

So Nature let you manage your business on the __Top perspective__,   Nature will take care of __Route__, __Distribute__,__Store__ and other things. that will reduce administrative costs greatly.

- Nature isolates business defines (__goals__) and business implements (__functions__), that will reduce as many constrains as possible between them, and then reduce the communication costs greatly in your team, let's elaborate on.
  - For managers:
    - No technology limitation but pure business, it make the thing is what it should be. 
    - All your goals are clear, visible, accurate and in a uniform form, they are easy to be shown to your team, whether the bird taking the big picture or Insight into the details. 
    - The most important thing is, you will __never lost your target__ among a big system.
  - For developer:
    - Development task is more easier: 
      - Only focus one goals once a time, need not to have a global view and understand the whole thing, though it is easy to present by Nature.
      - Need not to take care about data consistently and idempotent and other things. 
    - Easy to maintains:  all workpiece is pluggable, and you can do `Gated-Launch`.
    -  Another most important thing is , Nature will __speed up your development iterations__.
- More directly, Nature will save you time and money,  __bigger is cheaper__.

## Usage scenarios

Backend data process, that is __asynchronized__. I have a plan to handle data __synchronized__ but not now.

## Want to know more?

[A concrete example](doc\help\demo\demo.md)

[Concept](doc\help\concept.md)

[Architecture](doc\help\architecture.md)

[Reference](doc\help\reference.md)

