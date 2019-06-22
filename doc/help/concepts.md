# Concepts

The Nature System focus on **data** and **relations** between data.  Data are the  **business goals**, so relations can let you achieve goal one by one.  This is little like workflow or BPMN,  but they are two very different things. The last tell us to **do something**, yet the first tell us we **need something**. How-to-do is very complex. and what-your-need is simple and more important.  Nature let you focus on important things and let you manage them easily.

## Data

Data are immutable, when they are created you can't modify or delete it.

There are two kind of data: `meta` and `instance`. 

### meta

`meta` tell us what the business is and how to used it. It is like program language's `Class`.

`meta` use `key` property to identify each of them. It's a string type, so you can give any value you like to it. But there is an advice: use large-small separated domain as its value, for example "/sale/order".

### instance

Instance is `meta`'s runtime's form. It's like program language's `Object`

So a meta can have any number of it's instances.

## Relation

A relation will connect tow and only two `meta`s.  but one `meta` can connect to many other `mets`s by many relations.

A relation has direction, begin from **from**-`meta` end to **to**-`meta`.

A relation defined a `converter` to convert from **from**-`meta`'s instance' to **to**-`meta`'s instance.

### converter

Converter is a logical outside of Nature. is can be implemented in many forms.  up to now Nature support the following forms

- http | https
- local rust

Nature use converters to glue all logical together, and make them loose coupling and powerful



## unfinished












![Alt text](../img/%E5%B9%BB%E7%81%AF%E7%89%8710.jpg)
`Thing` is only a define, at run-time you must bind an `Instance` to `Thing`.
If a `Relation` exists between two `Thing`'s, 
`Nature` will give upstream `Instasnce` to a converter which is binding to the certain `Relation` and it is at anywhere outside the `Nature`.
Then the converter generate new `Instance`s for the downstream `Thing`.
    
Here the `Relation` made a great job: It locked the __Geographic and Time Coordinates__, 
they are important for `Natrue` to retry when failed the execution. 

![Alt text](../img/%E5%B9%BB%E7%81%AF%E7%89%8711.jpg)

Converter is just one simple function, and the converter achieve __only one__ goal for the `Nature`.
Converter do no need to know all the other goals in `Nature`,
`Nature` will take care of them by call the converters recursively.

![Alt text](../img/%E5%B9%BB%E7%81%AF%E7%89%8712.jpg)
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

![Alt text](../img/%E5%B9%BB%E7%81%AF%E7%89%8713.jpg)

`Nature` only insert `Instance` to db, no __deletion__ no __update__.
`Nature` use __state version__ to express any chang on an `instance` and not all `Thing` have states.
This feature let `Nature` run converter equally for any times, tt is __idempotent__.