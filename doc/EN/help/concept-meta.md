# Meta

`Meta` is your business domain. it is the identity of the business objects. when you input `instance` data to Nature, Nature will check it's `meta` info. So `meta` is like program language's `Class`.

## Business Object

They are what important Nature to handle. Each BO must has a `meta` to bind to, like 'Order', 'Sheet', 'Check', 'Apply' and other things.

You can express a BO by `MetaType`, `key` and `version`. for example :  "B:sale/order:1" we call it `meta_string`,  `meta_string` should be unique to identify the BO.

- "/B" is `metaType`
- "/sale/order" is `key`
- ":1" is version
- "B:sale/order" we name it `full_key`

## MetaType

The following table lists all `metaType`

| Type           | Prefix | Description                                                  |
| -------------- | ------ | ------------------------------------------------------------ |
| Business       | /B     | Used with BO, but need to register to `meta` table before to be used. |
| System         | /S     | Used by Nature itself                                        |
| Dynamic        | /D     | Used with BO, need not to register to `meta` table, it's used for defining workflow as `runtime` |
| Null           | /N     | The terminate for the end of the workflow. You need not to define it in `meta` table |
| Multi-Serial   | /R     | This will allow  converter return multi target meta of instances and process them serially |
| Multi-Parallel | M     | This will allow  converter return multi target meta |

## version

A BO definition can have many versions, the default is "1". this is very useful for you business changing, for example you may want to add "earnest" property to you "Order" business domain. In this case you needn't to give a new `meta` but a new version to simplify the view of your global BOs.

## State

BO can has states, for example, a order can has new, paid, picked, outbound, delivering states etcetera.  States can be grouped,  and states can be mutually exclusive. you can use the following symbols to form a `Meta`'s state property.

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

**Important note:** 

- each state's name should be unique, even they are in different groups.
- if you leave state field empty the BO would  be not a state. except you fill "{“is_state”:true}" to the config field.

## Meta Settings

```rust
pub struct MetaSetting {
    pub is_state: bool,
    pub master: Option<String>,
}
```

Your can save your settings by JSON format to config field of meta-table .

- is_state:  A counter BO is a state-meta but we cant name the states, in this case we can set this setting to `true`.
- master: indicate that this BO is an attachment of the master. this BO would use master's id and Nature will give master instance as input parameter to the converter based on this BO. and This is a magic for auto-convert.

## Store `Meta` data

`meta` must be defined in table "meta" except `Dynamic` and `Null` `MetaType`.

For example, if we would register an __Order__, the sql  might be :

```mysql
INSERT INTO meta
(full_key, description, version, states, fields, config)
VALUES('B:sale/order', 'order', 1, '', '', '{}');
```

## Limitations

Nature does not allow converter to return more than one `metaType` of instances except "M" and "/R" `metaType`.

**Only one** `state-instance`  can be returned by converter. because `state-instance` may cause state-version conflict and a re-convert will be needed. That is great complex for multi-instance to face to this problem.