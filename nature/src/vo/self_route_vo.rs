use std::convert::TryInto;
use crate::common::NatureError;

use crate::domain::{DynamicConverter, SelfRouteInstance};
use crate::vo::InstanceVO;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SelfRouteInsVO {
    pub instance: InstanceVO,
    pub converter: Vec<DynamicConverter>,
}

impl TryInto<SelfRouteInstance> for SelfRouteInsVO {
    type Error = NatureError;

    fn try_into(self) -> Result<SelfRouteInstance, Self::Error> {
        Ok(SelfRouteInstance {
            instance: self.instance.try_into()?,
            converter: self.converter.clone(),
        })
    }
}