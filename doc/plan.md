# plan

## for the nearer term

* use `min-max-heap` to support priority converter
* integrate test
    multi downstream
    callback
    batch_serial
    batch_parallel
* unit test:
  * store 
  * plan 
    - plan already exists
* test task
  * process line
    * dispatch
    * convert
    * serial
    * parallel
    * task
  * new converter info 
  * new call-out-parameter
* snowflake for 128 bit implement.
* StorTaskInfo
    need include next task id to use
* delay execute
* optimise : dispatch need not to save to task
* Thing's key support parametrization
* diesel 1.3 have compile problem with last nightly rust. 2018年5月26日
https://github.com/rust-lang/rust/issues/50825
https://github.com/rust-lang/rust/pull/51042
* implement sqlite
  * Mapping
  * Carrier
  * StorePlan
* diesel::result::Error => NatureError
* init system : check "/S/serial" whether defined, if not create one

### Unit and Integrate test 

#### Integrate test
* parallel
* context select
* status select
* before convert: target status include and exclude
* callback

### other

* call out
* add cache to mapping dao
* mapping 's from and to should unique. 

## future

* quick start
* support local converter
* give a demo
  * avoid conflict for N instance to 1 status instance: aggregation.
* support multiple rpc types, such as web and gRpc
* monitor point
* verify input fields of thing. report error for undefined fields
* converter support balance
* traffic limit
* introduce thread pool
* create mock **crate** for generate Lock and Return Value.
