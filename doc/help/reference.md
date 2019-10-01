# Reference

## Meta



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

  