# 附录：多个库房

现在模拟一个情景，我们已经有了一个自己的库房，因为业务扩展的需要现在需要增加一个库房。出于成本考虑，这个库房的业务由第三方来承接。但这里有个问题要解决：如何标记订单该由哪个库房生产呢？

- 可能的玩法：将库房相关的参数放到 `Instance.content` 中，程序员在下一节点编程提取并处理，既用编程的方式来处理这种分支流程。
- Nature 推荐的玩法：将库房相关的参数放到 `Instance.context`中，这样可以通过 Nature 的上下文选择技术以非编程方式进行流程控制,请参考 [relation.md](https://github.com/llxxbb/Nature/blob/master/doc/ZH/help/relation.md)。

为了简单起见，此演示只演示用到的技术，流程可能不具有实用价值。运行本示例前请用 demo-multi-warehouse.sql 进行数据初始化。

## 建立订单和库房的元数据

```mysql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'order', '', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'warehouse/self', '', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'warehouse/third', '', 1, '', '', '');
```

我们需要创建三个元数据，一个用于订单，一个用于自建库房，一个是第三方库房。注意：这里的订单和商城 Demo 不一样，这里是简化版的订单。

下面我们来定义流程：

```mysql
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:order:1', 'B:warehouse/self:1', '{"selector":{"context_all":["self"]},"executor":{"protocol":"localRust","url":"nature_demo:multi_warehouse"}}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:order:1', 'B:warehouse/third:1', '{"selector":{"context_all":["third"]},"executor":{"protocol":"localRust","url":"nature_demo:multi_warehouse"}}');
```

这里我们会看到之前没有使用过的新的 `selector`: `context_all`。其作用是：订单的上下文中如果有 “self” 就会创建 `warehouse/self` 实例， 如果订单上下文中如果含有 “third” 就会生成 `warehouse/third` 实例。如果 context 里同时含有 self 和 third 则会同时生成两个实例，当然这在库房情景中是一种错误的设置方式。

执行器的代码请参考：nature-demo::executor::emall::multi_warehouse

订单数据的输入请请看示例代码：nature-demo::multi_warehouse::multi_warehouse。

- **Nature 要点**：Nature 的`上下文选择器`只对`上下文`的 key 进行选择，不能对 value 进行选择。因为`上下文`的 Value 是用户自定义内容，为了减少复杂性及从性能上的考量，不对其进行选择。
- **Nature 要点**：关系里没有指定`执行器`，所以这两个关系的下游数据是 Nature 自动生成的。我们只需要输入订单就好。

让我们看下运行效果，启动：

```shell
nature.exe
cargo.exe test --color=always --package nature-demo --lib multi_warehouse::multi_warehouse
```

在 `multi_warehouse` 里一共提交了 A、B、C、D 四个订单，A的上下文是 self， B的上下文是 third， 订单 C 的上下文 是 self 和 third.  D没有上下文。

**Nature 要点**：订单 C 的 context 设置是错误的，因为订单不可能同时分配给两个库房，这里故为为之只是演示一个上游数据可以命中多个`选择器`。但这种使用方式在其它场景下可能会非常有用，如对用户的兴趣进行分类统计时，一条上游数据就需要同时匹配多条下游数据。

运行后的数据如下：

| ins_key                                   | content | context                         | from_key                            |
| ----------------------------------------- | ------- | ------------------------------- | ----------------------------------- |
| B:order:1\|1487267541922457568            | "A"     | {"self":"self"}                 |                                     |
| B:order:1\|5511124463593097989            | "B"     | {"third":"third"}               |                                     |
| B:order:1\|9688354360942590765            | "D"     |                                 |                                     |
| B:order:1\|13159812850727140160           | "C"     | {"self":"self","third":"third"} |                                     |
| B:warehouse/self:1\|11492880383379452140  | "C"     |                                 | B:order:1\|13159812850727140160\||0 |
| B:warehouse/self:1\|18375946125590662357  | "A"     |                                 | B:order:1\|1487267541922457568\||0  |
| B:warehouse/third:1\|11492880383379452140 | "C"     |                                 | B:order:1\|13159812850727140160\||0 |
| B:warehouse/third:1\|15760981570053939593 | "B"     |                                 | B:order:1\|5511124463593097989\||0  |
