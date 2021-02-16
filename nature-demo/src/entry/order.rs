#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Commodity {
    pub id: u32,
    pub name: String,
    pub price: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct SelectedCommodity {
    pub item: Commodity,
    pub num: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Order {
    pub user_id: u32,
    pub price: u32,
    pub items: Vec<SelectedCommodity>,
    pub address: String,
}