# Nature

English|[中文](README.md)

## What is Nature？

**High-level service governance platform**: Nature believes that the core contradiction of current service governance lies in the contradiction between the single responsibility of services and the complicated relationship between services, which makes it difficult to maintain. The essential problem of complicated relationship is the **direct control** between services, and the powerful way of control is cohesion rather than coupling; coupling implies equal cooperation rather than unequal control. Nature takes over control, thus improving service governance qualitatively. Please refer to: [Why is service governance so difficult? I think it's time for a change of treatment](https://www.cnblogs.com/llxxbb/p/serviceGovernance.html))

**process orchestration tool**: Nature can use data to orchestrate business processes. Trust your eyes, it is not choreographed by functions, because the data can go directly to the business core. Nature **connects the world with data** to prevent you from getting stuck in the quagmire of functionality and losing your goal. Nature replaces concrete and changeable control with simple and unified rules. Nature can help you extract all the business control logic problems and let your system evolve into a brain if you want. For Nature's views on nature, time and space, philosophy and mathematics, see [Nature Architecture thought](doc/EN/help/architecture.md).

**low-code platform**: non-functional development is the most time-consuming, most complex, most difficult to maintain and least revealing in business systems, and Nature can help you reduce the cost by 80%. Please refer to: [problem Analysis and Solutions of existing Development models](doc/ZH/natureBusinessValue.md).

Nature has the following technical features:

## Distributed Stream-Compute-Engine

Traditional stream-compute is proposed to solve the timeliness of data processing, emphasizing calculation logic, and emphasizing `map-reduce`. Nature is also a stream-compute framework, and its core processing mode is: **Data--map--> Data--map-->Data...**, which simplifies `map-reduce` to `map` (It is not that `reduce` is discarded, but it can be embedded into `map` for processing). This simplified mode allows us to more easily focus on the data itself rather than the process, so **Nature emphasize data**, but not calculation logic. This point will be further explained below.

## Business Control Center

**Data--map--> Data--map--> Data** is **Nature's runtime mode**. From a management point of view, we pay more attention to results rather than processes, so Nature simplifies this mode further, the map is removed, and the new mode becomes: **Data-->Data-->Data...**. This is **Nature's design-time mode**. This also proves that Nature is a data-centric Stream-Compute-Engine.

Nature uses `Meta` and `Relation` to represent data and the relationship between them respectively. In this way, within Nature system, only Nature knows what business the data represents and how to find downstream businesses through `Relation` for all business systems involved in collaboration, so Nature becomes the actual business control center.

## Dispatching Center

The `map` in Nature runtime mode corresponds to the `Executor` in `Relation`. Nature will follow the data flow in the design-time mode to schedule Executor. These schedules include pre-processing, post-processing, idempotence, delay processing, timing processing, and batch processing. If it is timeout or encounters an abnormal environment, Nature will retry multiple times under the retry strategy, and try its best to ensure the final consistency of the data. Even if the final retry fails or a logical exception is encountered, Nature will not discard the task, but will put it in the error data table. After problem fixed, you can re-import the data from the error data table to the task data table, and Nature will retry these tasks again until they are successfully completed.

## Low Code Platform

You can see that Nature integrates many heavyweight elements, and the purpose of integration is to simplify our programming and enable developers to better focus on the business itself. It is embodied in the following aspects:

- Data driven
  Traditional function-oriented development will mix business data, technical data, control data, and temporary data together, creating unnecessary coupling, and unintentionally increasing the complexity and maintenance cost of the system; worse more, the key business Data may be "kidnapped" by the system, which becomes bloated, inefficient and difficult to change.
  Nature's design-time model is entirely composed of business data, without the smallest amount of function, which ensures the purity, intuitiveness and simplicity of the business, and can guarantee the absolute control of the business on the system. Nature's `Executor` divides the entire business system into the smallest collaborative units that are not coupled with each other, ensuring the simplicity of development and maintenance.

- Significantly reduce interface and storage related work

  Nature’s business process control is achieved through [Configuration](doc/EN/help/relation.md), and developers do not need to care about upstream and downstream process control issues. Reflected in two aspects: one is the definition of the interface, compared with the complex and a large number of personalized interface definitions in traditional projects, Nature only provides a limited number of interfaces; the other is the use of interfaces. Traditional projects need to write logic in the code for the calling of the interfaces, but for Nature, except submitting data to Nature, all interface calls are handled by Nature, which greatly simplifies interface-related design, development, debugging and follow-up maintenance work.

  Similarly, Nature adopts a centralized and unified data storage mechanism, and developers do not need to care about the design of data tables and indexes and subsequent development and maintenance work.

- Significantly reduce non-functional development related work

  Nature has done a lot of work for the availability (such as idempotence, eventual consistency of data) and reliability (such as retries, exception records) for the system. In addition, Nature also supports the business in terms of scalability , Such as the version technology of `Meta`. According to Pareto Principle (the 80/20 Rule), these 80% of the heavy and important work can now be undertaken by Nature.

## how to use

We need to do the following

1. Create a [mysql](https://www.mysql.com/) or [mariadb](https://mariadb.org/) or [Tidb](https://pingcap.com/en/) database , And execute [schema.sql](shell/schema.sql)

2. Define multiple business objects in the [Meta](doc/EN/help/meta.md) data table, for example: we define two business objects, `Order` and `Order Account`

   ```sql
   INSERT INTO meta (full_key, description, version, states, fields, config) VALUES
   ('B', 'sale/order', 'order', 1, '', '', ''),
   ('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}'); 
   ```

3. In the [relation](doc/EN/help/relation.md) data table, define the relationship to associate multiple business objects together, and set an `Executor` in the `Relation` for converting (equivalent to `map` in stream-compute), for example there is a definition for `order` and `order account` above:

   ```sql
   INSERT INTO relation
   (from_meta, to_meta, settings)
   VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"states":{"add":["unpaid"]}}}');
   ```

4. Write code to implement the [Executor](doc/EN/help/executor.md) you defined above. If it is based on Http, please start it after completion. If it is based on class library, please put it in the same directory as nature.exe. For example, the logic for `order` and `order account` is:

   - Generate an `order account` business object
   - Extract the receivables of all commodities from the incoming `order` and record them as the receivables of the `order account`
   - Return the `Order Account` object to Nature to drive the next step of processing

5. Configure the `DATABASE_URL` property in the `.env` file to point to the database you created

6. Start natrue.exe and retry.exe.

7. post request to Nature, such as submitting the `order` data to Nature, please refer to

## Graphical Management User Interface

[Graphical Management Client](https://github.com/llxxbb/Nature-Manager-UI) can let you manage this system in the following modes:

### Relation Mode

In this mode, you can organize how the business to work.

![main](https://raw.githubusercontent.com/llxxbb/Nature-Manager-UI/main/doc/relation.png)

### Domain Mode

In this mode, you can organize the business domain.

![main](https://raw.githubusercontent.com/llxxbb/Nature-Manager-UI/main/doc/domain.png)

### Data-Flow Mode

In this mode, you can see the data flow

![main](https://raw.githubusercontent.com/llxxbb/Nature-Manager-UI/main/doc/instance.png)

## Learn more about Nature

 [Nature interface definition](doc/EN/help/nature-interface.md)

If you want to understand Nature's view of nature, time and space, mathematical meaning and philosophical meaning, please read: [Nature architecture](doc/EN/help/architecture.md)

If you want to learn how to apply Nature in actual situations read: [Example and function explanation](nature-demo/README_EN.md), [Solutions to some business scenarios](doc/EN/help/use-case.md)

If you want to know how Nature features technical features and how these features are implemented read: [Nature tecnology characteristics](doc/EN/help/characteristics.md)

If you want to understand differences between Nature and streaming computing, messaging systems, workflow etc. please read: [Compare with other framework](doc/EN/compare.md)

If you want to use Nature better, read: [Use Meta](doc/EN/help/meta.md), [Use Relation](doc/EN/help/relation.md), [Built-in executor](doc/EN/help/built-in.md)

## Other explanation

The main function of this project has been completed. Now I'm looking for cooperation. If you are interested, please email 309577603@qq.com, WeChat: llxxbb76