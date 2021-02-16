# Concrete examples

English|[中文](README.md)

At here we would build an Online-Shop based on Nature.  The project will involves order, pay, warehouse and delivery domain. Even more we make some statistics through multi-dimensions. 

Don't worry about the complexity, we start at simple first, then step by step to achieve the final target.  Even thou I think the code lines are great reduced compare to the traditional development, conservative estimate they are less than half.

## How to read it

If you are the first time to know Nature,  It's best to view this demo from top to bottom.

Each chapter include little key-points of Nature, this let you come to know Nature.

In the whole demo description. there are some sections titled with **"Nature key points"** that would mind your attention how to do the thing in Nature way.

## Let‘s begin

| chapter                                 | digest                                                    | key points                                                   |
| --------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------ |
| [prepare](../Nature/nature-demo/doc/EN/prepare.md)               | prepare for the demo                                      | how to run Nature                                            |
| [generate order](doc/EN/emall/emall-1-order-generate.md) | user commit an order into to Nature                       | `Meta`, master `meta`, define target-state, `Converter`  and how to commit business object to Nature |
| [pay for the bill](doc/EN/emall/emall-2-pay-the-bill.md) | user can pay many times for the big bill.                 | upstream select, state conflict control                      |
| [stock-out](doc/EN/emall/emall-3-stock-out.md)           | the warehouse system is slow to process the order's goods | input state instance, callback                               |
| [delivery](doc/EN/emall/emall-4-delivery.md)             | collaborate with the third-party                          | parameterization input                                       |
| [signed](doc/EN/emall/emall-5-signed.md)                 | user received the goods                                   | delay converter                                              |


The following unfinished yet.

| chapter                              | digest                                                       | key points                                |
| ------------------------------------ | ------------------------------------------------------------ | ----------------------------------------- |
| [sale statistics](doc/EN/emall/emall-6-statistics.md) | from goods view, make statistics freely, extensible, no coding. | context, embedded counter, serial process |
| user consumption data                | make data which can be got by user id, such as order list    | parallel process                          |



