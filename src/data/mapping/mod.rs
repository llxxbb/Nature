#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    from: ThingFrom,
    to: ThingTo,
    how: Converter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingFrom {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingTo {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Converter {}