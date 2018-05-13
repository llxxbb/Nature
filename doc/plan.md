# plan

## for the nearer term

* implement sqlite
  * Instance
    * get_last_status_by_id
    * source_stored
    * unit test

* change other test like thing_define_cache unit test to avoid parallel test problem
  
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



