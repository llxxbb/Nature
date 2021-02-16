# 订单拆分

订单数据是个非结构化的 json 数据，为了便于后续的统计，我们需要将订单中的商品解析出来并使用相对结构化的方式独立存储。sql 配置数据如下：

```mssql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/order', 'order', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/money', 'item money', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/count', 'item count', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('M', 'sale/order/to_item', '', 1, '', '', '{"multi_meta":["B:sale/item/count:1","B:sale/item/money:1"]}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'M:sale/order/to_item:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_to_item"}}');
```

`sale/order`: 用作 `relation` 输入源.

**Nature 要点**：我们在 `sale/order/loop` 配置里遇到一种新的 `Meta` 类型： `MetaType::Multi`，该类型可以允许 `relation` 同时输出多个不同 `Meta` 对应的实例的，但它只是一个虚拟类型，自身不会不会产生实质性的 `Instance`。`MetaType::Multi` 需要设置 multi_meta 属性，以限定可以产出的 `Meta` 实例，而且这些 `Meta` 必须被定义过。

**Nature 要点**：之所以引入`MetaType::Multi`是出于性能上的考虑。如果不使用 `MetaType::Multi` ，我们完全可以定义两个 `relation` 来分别生成商品的销量和销售额数据。但这样我们需要传输两次订单数据，解析两次订单数据，所以有较多无谓的资源浪费；而使用 `MetaType::Multi` 技术，我们只需传输和解析一次订单数据就可以了。

订单的输入请参考：nature-demo::sale_statistics::sale_statistics_test

`order_to_item` 执行器的代码主要是将一个订单数据转换成多条商品统计数据，每个商品分两个指标进行统计：销量和销售额。

运行下面的脚本

```shell
nature.exe
cargo.exe test --package nature-demo --lib sale_statistics::sale_statistics_test
```

让我们来看一下部分运行结果：

| ins_key                                                   | content                                                      |
| --------------------------------------------------------- | ------------------------------------------------------------ |
| B:sale/order:1\|3827f37003127855b32ea022daa04cd\|         | {"user_id":123,"price":1000,"items":[{"item":{"id":1,"name":"phone","price":800},"num":1},{"item":{"id":2,"name":"battery","price":100},"num":2}],"address":"a.b.c"} |
| B:sale/item/count:1\|0\|1/3827f37003127855b32ea022daa04cd | 1                                                            |
| B:sale/item/count:1\|0\|2/3827f37003127855b32ea022daa04cd | 2                                                            |
| B:sale/item/money:1\|0\|1/3827f37003127855b32ea022daa04cd | 800                                                          |
| B:sale/item/money:1\|0\|2/3827f37003127855b32ea022daa04cd | 200                                                          |

在这里我们看到`sale/item/count` 和 `sale/item/money`实例将商品ID和订单ID都放到了 `Instance.para` 里，这样序做的目的是便于以后查询和防止主键冲突。