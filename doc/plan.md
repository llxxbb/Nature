# plan

## for the nearer term

* integrate test
    * simple_input
        * task test
          * dilivery store
          * instance store
          * dispatch
    callback
    batch_serial
    batch_parallel
* test task
  * process line
    * dispatch
    * convert
    * serial
    * parallel
    * delivery
  * new converter info 
  * new call-out-parameter
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


### Retry carry

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
* give a demo
* support multiple rpc types, such as web and gRpc
* monitor point
* verify input fields of thing. report error for undefined fields
* converter support balance
* traffic limit
* introduce thread pool
* create mock **crate** for generate Lock and Return Value.
