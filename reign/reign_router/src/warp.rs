use crate::RouterTypeTrait;

pub struct RouterTypeWarp;

impl RouterTypeTrait for RouterTypeWarp {
    const TYPE: &'static str = "warp";
}
