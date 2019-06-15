# Nature

Nature Help you to manage your data from a __High Perspective__. and automatic converting one data to another data incessantly, this will be continuing until all data __assigned by you__ to be converted.

Sorry for the  __Nature__ word that named for this project.  Nature means all things,  surely this it can not do that, but I could not think out a suitable word for it. It can do all the topic's (__Workflow__, __Message Queue__, __Stream__, __BPM__, __Map Reduce__, __Gateway__, __Distributed_System__ and __Database__) works which I list for this project.

Please do not think it too complicated. there are only few things be introduced in this project: __data__ and __relation__ between data. Just like atoms and there relations make up the complex world, in this project, __data__ make up all your business. So I like the name __Nature__. 

__Important__:  this project is in a very earlier stage, it is far to mature.

## A big question

This project covered a lot of  fields, and each field have their own mature and good projects. why do I provide a new one? 

No, this project only focus on one field: __Data__. I have no intention of compelling with other fields, it's just in these fields, let us to see it deeper.

### Functions vs goals

__Workflow__, __Message Queue__, __Stream__, __BPM__, __Map Reduce__ are __function__ oriented, it may let business go, but when business grow very large, you may __lost__ in functions, because function __code-binding with the goal__, it can __hide goals(data) inadvertently__. So the business system be unstable to  try to  close its vague goals.

Nature let you to plan your __goals__ (data), it's clearly in there and not easily to be changed.  nature __"hide functions deliberately"__ for you to easy you work. So you will get a more stable business system.

### Bottom-up management vs top-down management

__Gateway__, __Distributed_System__ and __Database__ are __bottom-up management__. They are technology oriented, there manage their own affairs in their own way, so it's hard to unify and simplify the management.

Nature let you declare the relation  between goals  and who can do transfer between the goals.  This is all you need to do, other things nature will take it care.

- Nature like a __gateway__: You don't care the data go where and how to go.
- Nature like a __distributed system__: you don't care data availability and consistently.
- Nature like a __KV-database__:  you can query data you putted in and  Nature itself transferred, and your data will __never be lost and changed__ when they are written down.

So Nature let you manage your business on the __Top perspective__,   Nature will take care of __Route__, __Distribute__,__Store__ and other things. that will reduce administrative costs greatly.

## What's in it for you

- Nature isolates business defines (__goals__) and business implements (__functions__), that will reduce as many constrains as possible between them, and then reduce the communication costs greatly, let's elaborate on.
  - For managers:
    - No technology limitation but pure business, it make the thing is what it should be. 
    - All you goals are clear, visible, accurate and in a uniform form, they are easy to be shown on your monitor, whether the bird taking the big picture or Insight into the details. 
    - The most important thing is, you will __never lost your target__ among a big system.
  - For developer:
    - development task is more easier: Nature manages the goals and schedule the services which connected to it,  each service just need to achieve one goal, and need not to take care about data consistently and idempotently and store and other things. 
    - easy to maintains:  services is pluggable , developer need not to take care about the compatibility,  Nature will do it for you.
    -  Another most important thing is , Nature will __speed up your development iterations__.
- More directly, Nature will save you time and money,  __bigger is cheaper__.

## Usage scenarios

Backend data process, that is __asynchronized__. I have a plan to handle data __synchronized__ but not now.

## Want to know more?

[A concrete example](doc\help\demo\demo.md)

[Concept](doc\help\concept.md)

[Architecture](doc\help\architecture.md)

[Reference](doc\help\reference.md)

