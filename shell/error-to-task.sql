```mysql
insert into task
select `task_id`, `meta`, `data_type`,`data`,`last_state_version`,`create_time`,datetime('now','localtime'),0
from task_error

```
