#[derive(Debug, Clone)]
pub struct Mapping {
    from: ThingFrom,
    to: ThingTo,
    how: Converter,
}

#[derive(Debug, Clone)]
pub struct ThingFrom {}

#[derive(Debug, Clone)]
pub struct ThingTo {}

#[derive(Debug, Clone)]
pub struct Converter {}