# A concrete example
The whole code of this example you will be find at [Nature_Demo](https://github.com/llxxbb/Nature-Demo) project. 

Assume we want to implement an Online-Shop  system, and let's start at simple, the processing flow is shown as follow:

###### ![process flow](processing_flow.png)

## plan goals

This is the first step for manager, Let list what data we wanted.

![plan goals](plan_goals.png)

All this must defined in Nature. otherwise Nature will refuse to accept it. Don't be afraid of the class diagram, you need not to write any code, just fill these goals to Nature DB's table: `thing_defines`.  I had written the [sql](thing_defines.sql) for you

## Specify how and to achieve the goals

The second step is design path from one goal to another, let's see:

![how](how.png)



## runtime

多个库房的问题

多次中转的问题

