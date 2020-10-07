# Nature Data Definition

The data definitions used by the Nature interface and `Executor` interface are listed here

## ConverterParameter

Input parameters for the `convert` interface of `Executor`

**rust form**:

```rust
pub struct ConverterParameter {
     pub from: Instance, 	// upstream Instance, please refer to the following Instance
     pub last_state: Option<Instance>, // The last state of the current Instance
     pub task_id: u64, 		// The current task id, It is the credentials when call back for delay processing.
     pub master: Option<Instance>, // upstream mater Instance
     pub cfg: String, 		// is the value of Executor.settings, used to guide how the current Executor works, it is part of the settings field in the relation data table.
}
```

**json form**:

```json
{
     "from": {}, 		// Upstream Instance, please refer to the following Instance
     "last_state": {}, 	// Default null, the last state of the current Instance, please refer to the Instance below
     "task_id": 123, 	// The id of the current task, It is the credentials when call back for delay processing.
     "master": {}, 		// Default null, The upstream mater Instance, please refer to the following nstance json
     "cfg": "" 			// The value of Executor.settings, used to guide how the current Executor works, it is part of the settings field in the relation data table.
}
```

## Instance

[Meta](meta.md)‘s runtime

**rust form**:

```rust
pub struct Instance {
     pub id: u64, 			// unique identification
     pub data: BizObject, 	// the specific content of the business object
     pub create_time: i64, 
}
```

**json form**:

```json
{
    "id":12343, 			// unique identification
	"data": {}, 			// For the specific content of the business object, please refer to BizObject
	"create_time": 133231 	// notice: long type
}
```

## BizObject

Specific content of `Instance`

**rust form**:

```rust
pub struct BizObject {
    pub meta: String, 		// the meta-string form of the Meta belongs to
    pub content: String, 	// business specific content
    pub context: HashMap<String, String>, 		// other business information serving process control.
    pub sys_context: HashMap<String, String>, 	// Same as context except that the content in this context is regulated by Nature.
    pub states: HashSet<String>, 				// business status information. Its value must be defined in Meta before it can be used
    pub state_version: i32, 		// version of state information
    pub from: Option<FromInstance>, // upstream Instance, see below FromInstance
    pub para: String, 				// Another way to unique mark the current Instance, It is a powerful tool for correlating external data
}
```

**json form**:

```json
{
    "meta": "B:sale/order:1", 		// The meta-string form of the Meta belongs to
    "content": "item:[...],price:123,...", // business specific content
    "context": {"k1":"v1"...}, 		// Default null, Key-Value structure, Value is String type, serving other business information for process control.
    "sys_context": {"k1":"v1"...}, 	// Same as context except that the content in this context is regulated by Nature.
    "states": ["s1"...], 	// Default null. Status information of the business. Its value must be defined in Meta before it can be used
    "state_version": 0, 	// version of state information
    "from": {}, 			// Default null, upstream Instance, see below FromInstance
    "para": "", 			// Default null, another way to unique mark the current Instance, it is a powerful tool for linking external data
}
```

## FromInstance

Is a simplified Instance, used to represent upstream Instance

**rust form**:

```rust
pub struct FromInstance {
     pub id: u64, 			// Please refer to Instance.id above
     pub meta: String, 		// Please refer to BizObject.meta above
     pub para: String, 		// Please refer to BizObject.para above
     pub state_version: 0, 	// Please refer to BizObject.state_version above
}
```

**json form**:

```rust
{
     "id": 123, 				// Please refer to Instance.id
     "meta": "B:shop/cart:1", 	// Please refer to BizObject.meta above
     "para": null, 				// Please refer to BizObject.para above
     "state_version": 0, 		// Please refer to BizObject.state_version above
}
```

## ConverterReturned

The results of the `Executor`'s processing.

**rust form**:

```rust
pub enum ConverterReturned {
    LogicalError(String), 	// logic error, Nature will not retry
    EnvError(String), 		// Environment error, Nature will retry according to the retry strategy
    None, 					// No Instance generated
    Delay(u32), 			// Used for delay (seconds) processing, please see Demo for specific usage
    Instances(Vec<Instance>), 			// produced Instance
    SelfRoute(Vec<SelfRouteInstance>), 	// Produce Instance with dynamic routing
}
```

**json form**:

Return the generated `Instance`, where the `ins` attribute is an array of `Instance`, please see `Instance` above for structure description.

```json
{"type":"Instances","ins":[]}
```

If you don’t need to return any `Instance`, please return the following json

```json
{"type":"None"}
```

If you encounter a logic error, please return to the following json, and Nature will abandon the task!

```json
{"type":"LogicalError","msg":"some error"}
```

If you encounter an environmental error, please return the following json, and Nature will retry according to the retry strategy.

```json
{"type":"EnvError","msg":"some error"}
```

If you need to perform asynchronous processing, you can immediately return the following information to Nature, and Nature will not emit a retry within the time given by you. When complete the conversion task you should call back the `/callback` interface described in [Nature interface](nature-interface.md). The unit of `num` here is seconds.

```json
{"type":"Delay","num":10}
```

If you want to return Instance with your own route, please return the following form, where `ins` is the SelfRouteInstance array, and its structure is shown in the SelfRouteInstance below.

```json
{"type":"SelfRoute","ins":[]}
```

## SelfRouteInstance

`Instance` with its own routing information

**rust form**:

```rust
pub struct SelfRouteInstance {
     pub instance: Instance, 				// generated Instance
     pub converter: Vec<DynamicConverter>, 	// see DynamicConverter below
}
```

**json form**:

```rust
{
     "instance": {}, // The generated Instance, please refer to the instance above
     "converter": [] // is an array of DynamicConverter, please see DynamicConverter below
}
```

## DynamicConverter

Dynamic converter for `SelfRouteInstance`

**rust form**:

```rust
pub struct DynamicConverter {
     pub to: Option<String>, // Target Meta, only MetaType::Dynamic and MetaType::Null are supported
     pub fun: Executor, // Executor used to perform conversion task, see Executor below
     pub use_upstream_id: bool, // default false, whether to use upstream Instance.id as generated Instance id
     pub delay: i32, // default 0, Executor needs to delay given time, unit: second
}
```

**json form**:

```json
{
     "to": "D:targetMeta:1", // target Meta, only supports MetaType::Dynamic and MetaType::Null
     "fun": {}, // Executor used to perform conversion tasks, see Executor below
     "use_upstream_id": true,// default false, whether to use upstream Instance.id as generated Instance id
     "delay": 10 // Default 0, Executor needs to delay given time, unit: second
}
```

## Executor

Definition of Nature `Executor`

**rust form**:

```rust
pub struct Executor {
     pub protocol: Protocol, 	// see Protocol bellow
     pub url: String, 			// Executor coordinates
     pub settings: String,		// Executor's own configuration.
}
pub enum Protocol {
     LocalRust, 		// rust lib package
     Http,
     Https,
     Auto, 				// Nature will automatically generate Executor, you cannot use this protocol directly
     BuiltIn, 			// Use Nature's built-in Executor
}
```

**json form**:

```json
{
     "protocol": "http", // Optional protocol: http|https|localRust|builtin
     "url": String,		 // Executor coordinates
     "settings": String  // Executor's own configuration.
}
```

## DelayedInstances

parameters which `Executor` used to callback Nature

**rust form**:

```rust
pub struct DelayedInstances {
     pub task_id: u64, 				// Task ID assigned to Executor by Nature
     pub result: ConverterReturned, // see ConverterReturned above
}
```

**json form**:

```json
{
     "task_id": 123, 			// Task ID assigned to Executor by Nature
     "result": {} 				// See ConverterReturned above
}
```

## KeyCondition

Used to query Instance in Nature

**rust form**:

```rust
pub struct KeyCondition {
    pub id: u64, 		// instance id
    pub meta: String, 	// meta-string
    pub key_gt: String, // For batch query, the key of the result must be greater than this value
    pub key_ge: String, // For batch query, the key of the result must be greater than or equal to this value
    pub key_lt: String, // For batch query, the key of the result should be less than this value
    pub key_le: String, // For batch query, the key of the result must be less than or equal to this value
    pub para: String, 	// instance.para
    pub state_version: i32, 	// instance.state_version
    pub time_ge: Option<i64>, 	// The creation time must be greater than or equal to this value
    pub time_lt: Option<i64>, 	// The creation time must be less than this value
    pub limit: i32, 			// number of results returned
}
```

The composition of key is "meta-string|id|para"

**json form**:

```json
{
    "id": 123, 					// instance id
    "meta": "B:sale/Order:1", 	// meta-string
    "key_gt": "", 				// For batch query, the key of the result must be greater than this value
    "key_ge": "", 				// For batch query, the key of the result must be greater than or equal to this value
    "key_lt": "", 				// For batch query, the key of the result should be less than this value
    "key_le": "", 				// For batch query, the key of the result must be less than or equal to this value
    "para": "", 				// instance.para
    "state_version": 0, 		// instance.state_version
    "time_ge": 123 	// The creation time must be greater than or equal to this value
    "time_lt": 456, // The creation time must be less than this value
    "limit": 1, 	// Number of results returned
}
```