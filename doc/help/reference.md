# Reference

## Meta

each `meta` has a `key` used to identify it.

- hierarchy

Like 7 Layers of the OSI Model, we must put a problem into a layer, 

It let you to shrink it's range and make it easy to study, this is why I call it `Thing` but not layer.

data 's status

context

`Thing` is your business goal. 

Nature can accept a `Thing` and it's `Instance`s when they are registered to table `thing_defines`, except one `Thing Type`: __Dynamic__. and there are four `Thing Type`s.

For example, if we would register an __Order__, the sql  will be :

```sqlite
INSERT INTO thing_defines ("full_key",description,version,states,fields) VALUES (
'/B/Order',NULL,1,NULL,NULL);
```

__Notice__:  `full_key` is separated by __/__,  you can use it to organize you business from high level to low level. for example "/B/Sale/Order".

__Notice: __ `/B` of the '/B/Order' `full_key` is to state a `Thing Type`. The `Thing Type` must be included in the `full_key`, and first letter must be __/__ and the second letter is what the `Thing Type`  is for this `Thing Define`.

__Notice:__ the `version` field indicate the `Thing`'s  changelog. when `Thing` need to be changed, add a new version is ok,  it can be coexists with the old one.  this is very useful for gray deployment.

### Thing type

Nature support four `Thing Type`:

| Type     | Prefix | Description                                                  |
| -------- | ------ | ------------------------------------------------------------ |
| Business | /B     | Used by you, but need to register to `thing_defines` table before to be used. |
| System   | /S     | Used by Nature itself                                        |
| Dynamic  | /D     | Used by you, need not to register to `thing_defines` table, it's used for define workflow as `runtime` |
| Null     | /N     | The terminate `Thing` for the end of the workflow.           |

## Convert configuration

### <a id='settings' />Settings

```json
{
    "selector":{
        "source_status_include":[],
        "source_status_exclude":[],
        "target_status_include":[],
        "target_status_exclude":[],
        "context_include":[],
        "context_exclude":[]
    },
    "executor":[
        {
            "protocol":"",
            "url":"",
            "group":"",
            "proportion":1,
    	},
    ]
}
```

```
LOCALRUST|HTTP|HTTPS
```

### Protocol example

- Http

```json
{
    "protocol":"Http",
    "url":"http://some_domain:8081/some_converter"
}
```

- LocalRust

```json
{
    "protocol":"LocalRust",
    "url":"some_lib.dll:some_converter"
}
```

  