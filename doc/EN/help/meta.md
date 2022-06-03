# Meta

`Meta` is used to define business data, which is the design time of business data, and the runtime of business data is `Instance`

## Meta-String

A complete `Meta` consists of three parts: `MetaType`, `key` and `version`. For simplicity, Nature represent it by a string in the following form:

```
MetaType:key:version
```

`Meta-String` will be used as attributes such as `Instance.meta`. For example, the `Meta-String` of the "order" instance can be expressed as "B:sale/order:1".

### MetaType

`MetaType` supported by Nature list bellow:

| Type               | Description                                                                                                                                                                                                                                                                                                                                                              |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| (**B**)usiness     | Represents a **business** object. This type of `Meta` must be defined in the `meta` data table before it can be used.                                                                                                                                                                                                                                                    |
| (**S**)ystem       | Represents a **system** object, managed by Nature itself, no need to define it in the `meta` data table.                                                                                                                                                                                                                                                                 |
| (**D**)ynamic      | It is used to represent a **business** object defined by external dynamically at runtime, and does not need to be defined in the `meta` data table.                                                                                                                                                                                                                      |
| (**N**)ull         | Represents an object with no practical meaning, used when `Executor` has no output. No need to define in the `meta` data table.                                                                                                                                                                                                                                          |
| (**M**)ulti-Target | Under normal circumstances, Nature does not allow `Executor` to return multiple `Instance`s with different `meta` attributes, unless this type of `MetaType` is used. This type requires the `multi_meta` attribute to be defined in the settings, please see the description below. This type of `Meta` must be defined in the `meta` data table before it can be used. |
| (**L**)oop         | Create itself repeatedly until sys_context contains the FINISHED attribute. Generally used for batch processing. This type of `Meta` must be defined in the `meta` data table before it can be used.                                                                                                                                                                     |

### key

be used to uniquely distinguish it from other business objects. You can use "/" to classify the business domain, which is very helpful for the visual management of the business domain.

### version

The development of the business may change `Meta` often. In order not to affect the existing functions, Nature uses a different `version` to redefine a **"new business object"**, but maintains the consistency of their **business areas**, means that they have the same `key`.  So`version` is very helpful for business expansion.

## State

A business object can have status, for example, an order can have the following status:

```
new, paid, picked, outbound, delivering, signed
```

Whenever the state of `instance` changes, Nature does not overwrite the previous state, but adds a new `Instance` whose state version number would be increased by 1 based on the previous.

Nature’s state definition has a very powerful form of expression, and can construct very complex states, such as grouping and exclusiveness. In the grouping, you can also nest grouping and exclusiveness, and nesting grouping in exclusiveness.

For example, suppose we have four states, s1, s2, s3, and s4, and s1 contains four sub-states of s1-1, s1-2, s1-3, and s1-4, s1-3 and s1-4 are mutually exclusive, s2 and s3 are also mutually exclusive, so we can use the following string expression to express this complex state:

```
s1[s1-1,s1-2,s1-3|s1-4],s2|s3,s4
```

The expression used symbols defined as follows:

| symbol | description                                                                                               |
| ------ | --------------------------------------------------------------------------------------------------------- |
| ,      | Used to separate different states                                                                         |
| [,]    | Represents a state group, the states in the group are also separated by ","                               |
| \|     | Indicates that the states are mutually exclusive, and only one of them is allowed to exist in `Instance`. |

**important hint:** 

- The name of each state in the expression must be unique, even if they are in different groups.
- If the `state`  attribute of `Meta` is empty, then this `Meta` will be none-state. Unless explicitly specify "is_state" to true in `Meta`‘s setting. 

## Meta settings

The setting information of `Meta` is in JSON format, which is defined as follows:

```json
{
     "is_state": false,     // default false. If the `state` property of `Meta` is empty but want to be state, you can set this property to true. For example, a counter `Meta` needs to be state.
     "master": null,         // default null, see the description below
     "multi_meta": [],         // default null, see the description below
     "cache_saved": false,     // default false, see the description below
     "only_one": false,     // default false, see the description below
}
```

- master: is a `Meta-String` that points to another `Meta`. The `Instance` corresponding to the master has several functions: one is to transfer the `master` attribute to the `Executor`, which is very convenient for separating the basic information of the business from the state information, such as [example](https://github. com/llxxbb/Nature-Demo) the separation of order and order state data. The second is that its id will be used as the id of the current `instance`. The third is the basis for Nature to realize the automatic `Executor` magically. Note: If `use_upstream_id` is set in the setting of [`Relation`](relation.md), the id of the upstream `Instance` will be used first.

- multi_meta: is a `Meta-String` array. The `Meta` whose `MetaType` is M can allow `Executor` to return multiple `Instance` of different `Meta`. The `Meta` used for returning must be defined here, and must be defined as an independent `Meta`. **Note**: `multi_meta` cannot contain the state `Meta`. **Note**: If `multi_meta` has only one value (Generally common in `Meta` whose MetaType is L), `Executor` does not need to specify the `meta` attribute of the output `Instance`, Nature will automatically fill it; if `multi_meta` is more than one Value, the output of `Executor` must clearly give the value of `Instance.meta`.

- cache_saved: If true, the generated `Instance` will be cached for a short period time to avoid repeated writing to the database to improve efficiency. Common in situations where different upstream generate the same downstream, for example the `Instance` generated for time based statistical task in [Example](../../../nature-demo/README_EN.md). **Danger reminder**: Use this option incorrectly may consume a lot of memory or even overflow! You can set the `CACHE_SAVED_TIME` option in the `.env` file to change the cache time.

- only_one: Only valid for `Meta` whose `MetaType` is L, and is used to mark whether the Loop has only one downstream `Instance` output. If it is false, each call of Loop can generate multiple `Instance` of different Meta, and these Meta given by the `multi_meta` attribute. If true, Nature regards the currently defined `Meta` as a state `Meta`, which is used to store state data each time when Loop called (the content is the `Instance` of the `Meta` specified by `multi_meta`) to serve Next time Loop, note that in this case, `multi_meta` can only define one element. The reason for processing in this way because:
  
  - `multi_meta` cannot accept state data, because processing multiple state data at the same time is extremely complex for architecture support.
  - From the user's point of view, users do not expect the intermediate results of Loop, so there is no need for state data in `multi_meta`.

## Define `Meta`

The `Meta` data stored in the "meta" data table. The following is an example of "Order" `Meta`:

```sql
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/order', 'order', 1, '', '', '');
```

## Restrictions

If `Meta` is stateful, then `Executor` can only return an `instance`. This is because Nature's conflict handling of state data is more complicated, and it is difficult to guarantee the consistency of state data.
