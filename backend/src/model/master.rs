use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct OfcMast{
    pub _id:i32,
    pub ofc_name:String,
    pub ofc_ltcode:String,
    pub ofc_parent:i32,
}

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct MenuMast{
    pub _id:i32,
    pub menu_name:String,
    pub menu_link:String,
    pub menu_icon:String,
    pub menu_parent:i32,
}