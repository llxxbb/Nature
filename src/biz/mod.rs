///! A public lib for outer user call `world_connection`

struct DataDefineBase {
    biz: String,
    version: u32,
}

struct Data {
    define: DataDefineBase,
    content: String,
    context: String,
}
