# Nature

Nature now is in very earlier stage. 

## Why need Nature

To make data consistence is a not easy work for engineers, most of all the data transferred between services. We need check, redo and make sure the target services are idempotent, we waste much time and money on that things.

Nature can take care of the data consistence, but you need to give the data to Nature first. Nature is a proxy between services, and make the service easy to develop and maintenance.

Nature organize all the data into a web, you can see every data's flow in real-time. the most important point is, Nature let you orchestrate the date flow over the services, no technology but pure business. Nature will be a **Business Command Center**!
 
## Conception

Nature is a abstract from the real nature. It is a dynamic system which changes things and transforms things incessantly over time. 

### Things

Things are need to be changed or transformed. this is the aim why we need to program.

#### type

#### instance

status version

### transform

It controls how to change or transform the **Things**

## What should it do

## How to use

### before to use

#### ID generator

**Nature** use **Thing**'s MD5 hash code as id by default. 

Maybe you need a **Distribute ID Generator** like [Twitter's snowflake](https://github.com/twitter/snowflake/releases/tag/snowflake-2010) for storing large data in a distribute DB.

