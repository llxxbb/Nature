# Guide

## implement converter

**Input Parameter**

// TODO


## callback

**Input Parameter**

* carrier_id

// TODO

## Batch input

### parallel process
 
will ignore error while store instance.

### serial process

In this case, it should has a converter to process after all items in batch had stored.

Important to know the converter next to serial store:

* source must be "/S/serial"
* give a specific context name for "/S/serial"
  * which will be bind to "/S/serial" `Instance` by Nature.
  * Nature use this context name to select but your converter not other converter.
  * Advice: give the target biz name as this context name. 
The given context will contains two list as a Json:
```json
{
    succeeded_id: Vec<UuidBytes>,
    errors: Vec<String>,
}
```
