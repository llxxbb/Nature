# Meta

`Meta` is your business domain. it is the identity of the business objects. when you input `instance` data to Nature, Nature will check it's `meta` info. So `meta` is like program language's `Class`.

## Thing

They are your business that will be inputted to and output by `Nature`.

They can be anything your want to be processed, like 'Order', 'Sheet', 'Check', 'Apply' and other things.

## key

`Meta` use `key` property to identify each of `meta`. It's a string type, so you can give any value you like to it. But for convention, you should use **"/"** to separate you large-small domain as its value, like a file system's directory structure, for example "/sale/order".

## MetaType

`Meta` have types called `MetaType`, all types list here:

| Type     | Prefix | Description                                                  |
| -------- | ------ | ------------------------------------------------------------ |
| Business | /B     | Used by business, but need to register to `meta` table before to be used. |
| System   | /S     | Used by Nature itself                                        |
| Dynamic  | /D     | Used by business, need not to register to `meta` table, it's used for defining workflow as `runtime` |
| Null     | /N     | The terminate for the end of the workflow.                   |

## full_key

`full_key` = `MetaType` + `key`. If the key is "/sale/order" and the `MetaType` is "/B" then the `full_key` should be "/B/sale/order".

## version

The same `Meta` can have many version, the default is "1". this is very useful for you business change, for example you may want to add "earnest" property to you "Order" business domain. In this case you needn't to give a new `meta` but a new version to simplify the view of your global business objects.

## Identify a `meta`

`full_key` + `version` will locate the certain one unique `meta`. the string-form is `full_key`:`version`. an example : "/B/sale/order:1".

## State

Some `Meta` has states, for example, a order can new, paid, picked, outbound, delivering states etcetera.  States can be grouped,  and states can be mutually exclusive. you can use the following symbols to form a `Meta`'s state property.

| symbol | description                                                  |
| ------ | ------------------------------------------------------------ |
| ,      | **used to separate between states**                          |
| [,]    | express a grouped state, items are separated by ","          |
| \|     | states connected with "\|" that only one can be used for `instance`. |

A example：

Suppose we have s1,s2,s3,s4 four states, s1 has s1-1, s1-2, s1-3, s1-4 four sub-states, s1-3 and s1-4 are mutexes, s3 and s4 are mutexes too, this `meta`'s  state would be:

```
s1[s1-1,s1-2,s1-3|s1-4],s2,s3|s4
```

**Important note:** each state's name can't be same, even they are in different groups.

## Store `Meta` data

`meta` must be defined in table "meta" except `Dynamic` `MetaType`.

For example, if we would register an __Order__, the sql  might be :

```sqlite
INSERT INTO meta ("full_key",description,version,states,fields) VALUES (
'/B/Order',NULL,1,NULL,NULL);
```

