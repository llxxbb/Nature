use super::*;

pub struct ProcessLine;

impl ProcessLine {
    pub fn store(_instance: Instance, _root: Root) -> Result<[u8; 16]> {
        Ok([11, 172, 237, 228, 64, 20, 63, 157, 183, 32, 23, 63, 104, 161, 201, 51])
    }
}

