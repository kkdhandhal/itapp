use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct user{
    pub _id:i32,
    pub usr_name:String,
    pub usr_pass:String,
    pub usr_icon:String,
    pub usr_full_name:String,
    pub usr_add1:String,
    pub usr_add2:String,
    pub usr_city:String,
    pub usr_state:String,
    pub usr_phone_no:String,
    pub usr_mob_no:String,
    pub usr_email:String,
    pub usr_menu_id:i32,
    pub menu_disp_id:Vec<i32>,
}