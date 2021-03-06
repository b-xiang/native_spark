use super::*;
// not necessary I guess
use downcast_rs::DowncastSync;
use objekt;
pub struct SplitStruct {
    index: usize,
}

pub trait Split: DowncastSync + objekt::Clone + Serialize + Deserialize {
    fn get_index(&self) -> usize;
}
impl_downcast!(Split);
objekt::clone_trait_object!(Split);
