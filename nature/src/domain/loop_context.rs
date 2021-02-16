#[derive(Serialize, Deserialize, Debug)]
pub struct LoopContext {
    pub next: String,
    pub len: usize,
}