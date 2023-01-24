# 异常的任务测试

## 数据初始化

在测试前需要初始化一下测试数据，执行下面的sql

```sql
truncate meta;
truncate instances ;
truncate relation ;
truncate task;
truncate task_error;

INSERT INTO nature.task_error
(task_key, task_type, task_for, `data`, create_time, msg)
values
( 'B:sale/order:1|6548010827999420547||0', 1, 'B:sale/order:1',  '{"instance":{"id":6548010827999420547,"path":{"meta":"B:sale/order:1"},"data":{"content":"{\\"user_id\\":123,\\"price\\":1000,\\"items\\":[{\\"item\\":{\\"id\\":1,\\"name\\":\\"phone\\",\\"price\\":800},\\"num\\":1},{\\"item\\":{\\"id\\":2,\\"name\\":\\"battery\\",\\"price\\":100},\\"num\\":2}],\\"address\\":\\"a.b.c\\"}"},"create_time":1637483762984},"next_mission":[{"to":"B:finance/orderAccount:1","executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target_demand":{"state_add":["unpaid"]}},{"to":"B:sale/orderState:1","executor":{"protocol":"auto"},"target_demand":{"state_add":["new"]}}]}','2021-12-05',''),
( 'B:sale/order:1|6548010827999420547||0', 2, 'B:finance/orderAccount:1',  '{"to":"B:finance/orderAccount:1","executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target_demand":{"state_add":["unpaid"]}}','2021-12-05',''),
( 'B:sale/order:1|6548010827999420547||0', 2, 'B:sale/orderState:1',  '{"to":"B:sale/orderState:1","executor":{"protocol":"auto"},"target_demand":{"state_add":["new"]}}','2021-12-05','');
```

执行 [demo-emall](../../nature-demo/doc/demo-emall.sql)

## 启动 manager.exe 以接受命令输入

## 启动 retry.exe 用于观察任务是否激活成功

## 启动测试脚本 failed_task.sh

## 数据表验证

查看 task 数据表里是否有 task_id  为1，2，3 的三条记录
