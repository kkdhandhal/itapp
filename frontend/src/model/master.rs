use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct OfcMast{
    pub _id:i32,
    pub ofc_name:String,
    pub ofc_ltcode:String,
    pub ofc_parent:i32,
}

