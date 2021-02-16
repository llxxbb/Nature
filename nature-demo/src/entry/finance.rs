#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Payment {
    pub order: String,
    pub from_account: String,
    pub paid: u32,
    pub pay_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct OrderAccount {
    pub receivable: u32,
    /// can not be over the receivable, the extra money would be record to the field `diff`
    /// design in this way can hold each pay which is over
    pub total_paid: u32,
    pub last_paid: u32,
    /// record the reason for account change
    pub reason: OrderAccountReason,
    /// positive: over paid, negative : debt
    pub diff: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OrderAccountReason {
    NewOrder,
    Pay,
    CancelOrder,
}

impl Default for OrderAccountReason {
    fn default() -> Self {
        OrderAccountReason::Pay
    }
}