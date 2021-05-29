# Nature

English|[中文](README.md)

## What is Nature？

Nature allows you to **fully understand, control and plan data** at the macro level, and present production data to decision makers in the most intuitive and concise way. It can orchestrate **data flow**, which is a system-understandable **strongly constrained requirement**, which can directly affect the development of application systems, thus avoiding the uncertainty of traditional requirements and the cost of communication and transformation. Note that this is not a functional orchestration. Nature **connects the world with data** to prevent you from getting stuck in a functional quagmire and losing your goal, so your management will be very direct, concise and effective.

Nature believes that the **core contradiction of current service governance** lies in the contradiction between the single responsibility of services and the large number of services and complex interactions. Although the concept of central platform has emerged to make the interaction have a clear hierarchical layering, it is only the organization of the business and can not bring essential simplification in the form of technology. because the essential problem of **complex interaction** is the **direct control** between services, and the powerful way of control is cohesion rather than coupling, but the invocation between services precisely reflects coupling rather than cohesion. Because services are independent individuals and equal in status, there is a contradiction that the will is to control but the form is cooperation. Please refer to: [Why is service governance so difficult? I think it's time for a change of treatment](https://www.cnblogs.com/llxxbb/p/serviceGovernance.html)

Nature takes a different approach, getting rid of service restrictions and goal-oriented, and abstracts **diversified functions** into converters, because the essence of the system work is to string different small goals together to achieve our final demands, and the converter performs the responsibility of "stringing together". The goal here is the data, and the converter is the function; Nature manages the data and the symbolized converter, so that Nature can separate the core process control from the traditional service and form an independent control center: the brain. This can greatly simplify the overall complexity of the system. Nature provides a new set of simple, unified and standardized development model for this purpose. For the architecture thought, nature view, time-space view, philosophy view and mathematics view of Nature, please see: [Nature Architecture thought](doc/ZH/help/architecture.md).

With this unified and standardized model, we can standardize some functions that previously need to be customized, which are often the most time-consuming, complex, difficult to maintain and least revealing in the business system, such as idempotence, retry, high concurrency, final data consistency, and so on. This will further simplify your services and greatly improve the robustness and maintainability of the entire system. Please refer to: [problem Analysis and Solutions of existing Development models](doc/ZH/natureBusinessValue.md).

## Theoretical basis

We only set up the system to process the data. Google, the massively parallel processing of data, gives a MapReduce scheme. Nature has improved on this basis, its processing mode is: **data --map--> data--map--> data...** It simplifies MapReduce to Map, Nature does not discard reduce,, in Nature's view, reduce is a mapping of multiple data to a piece of data and can be regarded as a special case of Map. This simplified pattern makes it easier to focus on data and the relationships between them, rather than on how to deal with them.

Because map is an action, it contains the meaning of transformation, it is functionalized is the abstraction of function by Nature, and Nature uses it to control the data flow during operation, so **data -- map--> data is the runtime mode of Nature**. But from a management point of view, we focus more on the results than on the process. So Nature further simplifies the schema, removing the map, and the new schema becomes: **data --> data --> data**. This is the **design-time mode of Nature**. This pattern allows us to focus more succinctly on the internal relationships between data, which are the basis for the operation of the system.

In other words, the relationship between data and data is the top-level design of the whole system, and it is the guidance and constraint to the realization of the function. On this basis, as long as we add a map function to the relationship, the system can run. In this way, any complex process can be concatenated through this simple mechanism to form functions. Nature uses `Meta` to represent data, `Relation` to represent the relationship between data, and `Executor` to indicate that map,Nature mainly uses these three elements to control the operation of the entire system.

## keywords
Data governance, service governance, process orchestration, low code platform

## Technical Features
- **High-order reusing**: unlike the reuse of methods, modules, classes in a program, which can be docked only through coding, but Nature reuse does not need coding. For example, notification capability can be configured to interface with audit, reminder, promotion and other different business areas. Nature has this capability because Nature standardizes the data. 
- **short process**: since an `Relation` is a process, it is very lightweight and provides great flexibility for process stitching. 
- **Fast business iteration**: Nature provides version capability for `Meta`, based on which you can build new processes and facilitate switching without affecting existing processes, which makes it very easy for you to adjust your business. 
- **data immutability**: Nature only adds data but does not change data, because changes may lead to data inconsistencies. For state data Nature provides version capabilities for changes to support the immutability of the data, and this is done by Nature itself. 
- **traceability of data processing**: the Nature runtime automatically records the relationship between data instances, making all data traceable. 
- **automatic retry**: Nature defines and regulates environment exceptions and logic exceptions, and automatically redoes the task when an environment exception is encountered. 
- **error data transfer**: Nature will not lose any data. Even if it encounters a logic error, it can save it for review. After troubleshooting, you can re-import the interrupted tasks into the task data table, and Nature will execute these tasks again until they are completed successfully. 
- **delay and timing processing**: Nature allows you to specify the time for data conversion, which is very meaningful for statistical analysis of data. 
- **idempotent**: Nature has no idempotent requirements for all systems involved in data conversion cooperation, but Nature will remember and use the results of their first successful processing, based on which Nature achieves its own idempotency. 
- **data consistency**: Nature supports the final consistency of data. 
- **selector**: the selector provides conditions for triggering data conversion. 
  Nature provides flexible and extensible selector settings. 
- **Executor**: Nature provides extensible executors based on different protocols. Currently, both rust local package and http executors are supported. 
- **pre-processing**: used for pre-data-conversion processing, such as data format adjustment. 
  Nature provides some built-in preprocessors, and you can also define your own preprocessors. 
  This is generally a technical process without introducing another business object: `Meta`. 
- **Post processing**: it is the same as the pre-processor, but it is only applied to the converted data processing. 
- **batch**: Nature provides a mechanism for managing the intermediate state in the process of massive data processing, so that the processing can be carried out in batches. 
  For a detailed description of the technical features, please refer to [Technical Features of Nature](doc/ZH/help/characteristics.md)

## impact on existing development models. 
It is mainly reflected in the following aspects. 

- **data-driven** (goal-driven): traditional development is mostly function-oriented, and functions generally provide services through interfaces. There are often a large number of interfaces in a system, so it is difficult to develop and maintain. And it is easy to make people fall into a functional whirlpool and lose their goals, especially the lack of large-scale collaborative systems from a global perspective. 
  On the other hand, Nature eliminates the interference of the function, makes the goal concise and comprehensive, and ensures the business's **absolute contro**l over the system. At the same time, there is no need to worry too much about the realization of the function. 
- **data purification**: traditional development can easily lead to **business data, technical data, control data and temporary data mixed together**, resulting in unnecessary coupling and sometimes difficult to separate, inadvertently increasing the complexity and maintenance costs of the system, to make matters worse, critical business data may be "kidnapped" by the system, and the system becomes bloated, inefficient and difficult to change. While the design-time pattern of Nature is composed entirely of business data, without any function, which ensures the purity, intuition and simplicity of the business.

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

![relation.png (1769×1036) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/relation.png?raw=true)

### Domain Mode

In this mode, you can organize the business domain.

![domain.png (933×428) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/domain.png?raw=true)

### Data-Flow Mode

In this mode, you can see the data flow

![instance.png (1507×1068) (gitee.com)](https://gitee.com/xb_li/Nature-Manager-UI/raw/dev/doc/instance.png?raw=true)

## Learn more about Nature

 [Nature interface definition](doc/EN/help/nature-interface.md)

If you want to understand Nature's view of nature, time and space, mathematical meaning and philosophical meaning, please read: [Nature architecture](doc/EN/help/architecture.md)

If you want to learn how to apply Nature in actual situations read: [Example and function explanation](nature-demo/README_EN.md), [Solutions to some business scenarios](doc/EN/help/use-case.md)

If you want to know how Nature features technical features and how these features are implemented read: [Nature tecnology characteristics](doc/EN/help/characteristics.md)

If you want to understand differences between Nature and streaming computing, messaging systems, workflow etc. please read: [Compare with other framework](doc/EN/compare.md)

If you want to use Nature better, read: [Use Meta](doc/EN/help/meta.md), [Use Relation](doc/EN/help/relation.md), [Built-in executor](doc/EN/help/built-in.md)

## Other explanation

The main function of this project has been completed. Now I'm looking for cooperation. If you are interested, please email 309577603@qq.com, WeChat: llxxbb76

## Roadmap

### core functions

#### meta

- add the field name: friendly name. 
- meta template: the meta type is T (emplate), which is used to provide reused meta information. 
  Meta of template type cannot generate Instance, the states, fields, and config can be used as templates. Other meta can use the `from` attribute to associate a template, or you can associate the non-template meta, which is a specification rather than a mandatory measure, and the template user can overwrite or append the content. 
- accompanying meta: is used to represent instances with bidirectional relationships. For example, a user belongs to a group and there is a user in the group. Concomitant objects are used to facilitate data retrieval. Concomitant objects are automatically maintained by Nature and no additional relation is required. It is in the form of [meta_name]_R, where R is the first letter of Reverse. Condition: para has two and only two parameters. 
- the lifecycle of instance can be defined to facilitate the automation of data migration. 
- Private status: used to reduce the number of domain objects. For example, order and order status can be managed by one domain object in the business. It is in the form of [meta_name]_S, and you can define the initial state. What corresponds to the private status is that the independent status meta, such as [responsible person] meta, [responsible person] can be changed, so [responsible person] is a status data.

#### relation
- add field: description, describes the role of the relationship. 
- relation template: the template only provides information in settings except `is_template` and `from`. Please set the `from` attribute to an exists relation.

#### selector

Make it convenient to support external extensions.

#### built-in converter

Random generator: can specify the number of digits generated, in the form of optional numbers, letters + numbers, letters

#### database

Support for elasticSearch, mongodb.

### demo

To show that Nature can avoid coding, We will try to implement the most commonly used and complex enough business scenario: auditing,  to dig the potential of Nature's non-coding capability out.
we will simulate the following audit scenario: 

- Business type: leave, reimbursement, loan. 
- Audit method: multi-level audit, multi-person audit, agent audit, designated leader audit.

### GUI

- Support the creation of `Meta` and `Relation`
- `Meta template` can be marked and be filtered  to be shown.
- You can copy from a `Meta`.
- Prompt whether to generate a new version of `Meta` when `Meta` changed
- The `companion-meta` will not be displayed in the GUI as a separate `Meta`, but the primary object to which it is attached will be marked
- `private-state-meta` is not displayed independently, but the master object to which it depends is marked. When creating a relationship, you need to choose whether to create it from master data or private state data.
- You can copy from a `Relation`. 