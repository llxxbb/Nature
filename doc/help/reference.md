# Reference



## Thing define

### Thing type

Nature support four `Thing Type`:

| Type     | Prefix | Description                                                  |
| -------- | ------ | ------------------------------------------------------------ |
| Business | /B     | Used by you, but need to register to `thing_defines` table before to be used. |
| System   | /S     | Used by Nature itself                                        |
| Dynamic  | /D     | Used by you, need not to register to `thing_defines` table, it's used for define workflow as `runtime` |
| Null     | /N     | The terminate `Thing` for the end of the workflow.           |

The type's prefix is used as `Thing`' full-key's first part. for example, a order thing's full-key is "/B/Order".

## Convert configuration

