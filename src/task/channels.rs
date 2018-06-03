use super::*;
use util::*;

lazy_static! {
    pub static ref CHANNEL_ROUTE : Channel<Carrier<StoreInfo>> = Channel::new();
    pub static ref CHANNEL_DISPATCH : Channel<Carrier<RouteInfo>> = Channel::new();
    pub static ref CHANNEL_CONVERT : Channel<Carrier<ConverterInfo>> = Channel::new();
    pub static ref CHANNEL_STORE : Channel<Carrier<StoreInfo>> = Channel::new();
    pub static ref CHANNEL_PARALLEL : Channel<Carrier<ParallelBatchInstance>> = Channel::new();
    pub static ref CHANNEL_SERIAL : Channel<Carrier<SerialBatchInstance>> = Channel::new();
}

