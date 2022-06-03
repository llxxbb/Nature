# Nature

English|[中文](README.md)

## One sentence introduction to nature

**Nature** is a development support platform, which is applicable to the development of large business systems. It separates the data definition from the code, thus realizing **the decoupling of data and system**.

## Core problems to be solved by nature

- Data island: the function driven development mode conceals and deprives the dominant position of the data. In the service-oriented scenario, the data acts independently, and the data cannot be uniformly defined and managed, resulting in problems such as repeated data definitions and unclear data boundaries.

- The system is difficult to maintain:  **the data definition is frozen by the code**, which makes it very difficult to adjust the data, especially when coupling multiple systems.

- It is difficult to reuse high-level non functional codes: such as data consistency, data invariance, data traceability, idempotence, environment exception auto retry, delayed execution, etc. these functions generally involve the collaborative processing of multiple data. The dispersion and diversity of data make the work very difficult to unify.

## Nature's solution

### How data is defined

In the traditional way, we find a data definition: we first need to find the system, and then view the interface document of the system, then we'll find it in the parameter description of the interface document. That is **find data through functions**, which is an inefficient management method. Because **data is a bridge for collaboration between systems**, and data is **shared** by multiple systems, it is inappropriate to **limit**the data definition  to the definition of an interface parameter. On the one hand, the idea of the development team would have limitations. On the other hand, once it is solidified in this form, it will be very difficult to make changes later. This is the root cause of **high maintenance costs**.

Since it is shared, the data definitions should be independent of each system and placed in one place to make them **explicitly visible** for easy reference. In addition, it is also necessary to **take the data definition as the top-level Organization**, rather than the system as the top-level organization. This is also in line with people's way of thinking: **goal oriented**. Data is the purpose, while the system is only a way to achieve it. This will help us **filter out a large amount of interference information** and effectively focus on target management.

Thus, Nature was born, and the definition of data can be centrally managed. How can nature help with these systems? Is it just a validation center for data definitions? This is obviously not enough. Now is the age of the data, one of the difficulties in management is **management of the complex relationships between data**.

### Relationship between data

In large-scale systems, considering the data volume, concurrency and other factors, we usually split the database vertically and horizontally. At the same time, we have deliberately avoided the use of `join`. The split makes us lose the global perspective, and then lose the ability to manage the global data. That is to say, we are getting away from the concept of **relational database**!

However, the relationship between data is very important for business! The **lack of visibility of this relationship** directly leads to high team collaboration costs and system maintenance costs. Therefore, it is necessary to reestablish the relationship between these data in some way to guide and constrain the development of the system. So nature did the second thing: to manage the relationship between data.

### Data consistency

Data consistency is a guarantee that the system **maintains the correctness of data relations** during operation. Generally, we hope that the transaction of the database can be guaranteed, but the distributed characteristic of the business system makes us have to face it by ourselves. This is a work with a high technical threshold. It is difficult to have a ready to use solution to deal with it, and the maintainability is poor.
With the help of **abstraction** of data and the relationship between them, Nature can **once and for all** deal with the consistency of data in a unified way, which requires Nature to have complete **control** over data processing.

### Data processing engine

When Nature takes over the control of data processing, business systems can communicate with each other can only through Nature. In this way, there will be no coupling between systems, the responsibilities of the system will be single, the technical threshold will be lower, and the function iteration and maintenance will be easier.

Nature only focuses on data. From the definition of data, the relationship between data, and ensuring the correct expression of data relations, that is, Nature is a data processing engine, which **subverts the traditional model of technology led data**, and on the contrary, uses data to regulate and restrict the behavior of the system.

## How Nature works

The data definition in Nature is called `meta`, which represents **a certain type of data** in business, such as order data type. Each `meta` has its own unique identifier. All `meta` forms a single tree with multiple levels in the form of **business domains**, thus forming the entire business layout of the enterprise and providing managers with a **static perspective** that can take a holistic view.

Corresponding to `meta` is the data `instance`, instance generated at runtime, which represents **a certain amount of data** in the business, such as an order data. Each `instance` belongs to only one `meta`.

The relationship between data in Nature is called `relation`, which is used to express the **upstream and downstream relationship** of two `meta`, `relation` provides managers with a **dynamic perspective** of the overall situation; You can specify `executor` on `relation` to convert an upstream `instance` to a downstream `instance`. For example, there can be a `relationship` between an order and an invoice. As for how to generate an invoice data based on an order, it is the work of the `executor`. The `executor` is scheduled by Nature at the appropriate time.

## What is the connotation of Nature

The objective world of nature is composed of things and the interaction (relationship) between things. This system is also built based on these two points, so it is named Nature. On the other hand, the system obeys the law of natural selection and governs by inaction. It uses selection instead of control to fit the most primitive, simple and powerful operating law of things in the ecosystem. It is reflected in:

- In the era of highly decentralized and large-scale collaborative development, participants have equal status, and mutual choice is much more efficient than control.

- In terms of complexity, control requires feedback, and selection does not require feedback, so it is simpler than control. When the scale of the system continues to expand, the difference in complexity will be more prominent.

## keywords

Data governance, service governance, process orchestration, low code platform, data island, requirement change.

## Technical Features

- **High order reuse**: Because data definitions exist independently of code, we can better design and adjust business data to make it easier to achieve commonality. At the same time, each `relation` is a minimum reusable unit, which not only enhances flexibility, but also further improves the reusability of services. For example, we can easily make the notification capability undertake audit, reminder, promotion and other different business areas. 
- **Fast business iteration**: On the one hand, the centralized and unified data definition will greatly reduce the communication cost; On the other hand, nature breaks the coupling between systems, so that iteration does not have so many constraints. Both of these will accelerate the iteration. 
- **Data immutability and traceability**: nature only adds data, does not change data, keeps data inconsistent, and provides data traceability.
- **Built in non functional capabilities**: automatic retry of environment exceptions, logical exception dump, delay processing, timing processing, idempotence, final data consistency, etc
- **Plug in**: Nature is a platform that supports flexible extensions. You can freely extend selectors, executors, pre and post processes, etc.
- **Batch processing**: Nature provides a mechanism for batch processing of massive data.
- **Context**: it include user context and system context. Some special data can be transferred in the process through the context.

For a detailed description of the technical features, please refer to [Technical Features of Nature](doc/ZH/help/characteristics.md)

## Quick start

We need to do the following

**start Nature**:

1. Create a [mysql](https://www.mysql.com/) or [mariadb](https://mariadb.org/) or [Tidb](https://pingcap.com/en/) database , And execute [schema.sql](shell/schema.sql)

2. Edit the  `.env` file, set `DATABASE_URL` property to the database that your installed.

3. Start natrue.exe，retry.exe and manager.exe。

**develop based on Nature**：

1. Define multiple business objects in the [Meta](doc/EN/help/meta.md) data table, for example: we define two business objects, `Order` and `Order Account`
   
   ```sql
   INSERT INTO meta (full_key, description, version, states, fields, config) VALUES
   ('B', 'sale/order', 'order', 1, '', '', ''),
   ('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}'); 
   ```

2. In the [relation](doc/EN/help/relation.md) data table, define the relationship to associate multiple business objects together, and set an `Executor` in the `Relation` for converting (equivalent to `map` in stream-compute), for example there is a definition for `order` and `order account` above:
   
   ```sql
   INSERT INTO relation
   (from_meta, to_meta, settings)
   VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"states":{"add":["unpaid"]}}}');
   ```

3. Write code to implement the [Executor](doc/EN/help/executor.md) you defined above. If it is based on Http, please start it after completion. If it is based on class library, please put it in the same directory as nature.exe. For example, the logic for `order` and `order account` is:
   
   - Generate an `order account` business object
   - Extract the receivables of all commodities from the incoming `order` and record them as the receivables of the `order account`
   - Return the `Order Account` object to Nature to drive the next step of processing

**Run**：

post request to Nature, such as submitting the `order` data to Nature, Nature will automatically drive the `executor` in order to complete the defined tasks.

## Nature details

- [Use Meta](doc/EN/help/meta.md)

- [Use Relation](doc/EN/help/relation.md)

- [Built-in executor](doc/EN/help/built-in.md)

- [tasks](doc/EN/help/task.md)

- [Nature's interface](doc/EN/help/nature-interface.md)

**About javascript**:
JS will have accuracy problems when handling i64 or U64. For this reason, `nature.exe` and `manager.exe` provides corresponding interfaces with the suffix JS. These interfaces use string instead of U64 or i64.

## Graphical Management User Interface

[Graphical Management Client](https://github.com/llxxbb/Nature-Manager-UI) can let you manage this system in the following modes:

### Relation Mode

In this mode, you can organize how the business to work.

![relation.png (1769×1036) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/relation.png?raw=true)

### Domain Mode

In this mode, you can organize the business domain.

![domain.png (933×428) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/domain.png?raw=true)

### Data-Flow Mode

In this mode, you can see the data flow

![instance.png (1507×1068) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/instance.png?raw=true)

## Learn more about Nature

If you want to understand Nature's view of nature, time and space, mathematical meaning and philosophical meaning, please read: [Nature architecture](doc/EN/help/architecture.md)

If you want to learn how to apply Nature in actual situations read: [Example and function explanation](nature-demo/README_EN.md), [Solutions to some business scenarios](doc/EN/help/use-case.md)

If you want to know how Nature features technical features and how these features are implemented read: [Nature tecnology characteristics](doc/EN/help/characteristics.md)

If you want to understand differences between Nature and streaming computing, messaging systems, workflow etc. please read: [Compare with other framework](doc/EN/compare.md)

## Other explanation

The main function of this project has been completed. Now I'm looking for cooperation. If you are interested, please email 309577603@qq.com, WeChat: llxxbb76

[release log](doc/release/release.md)