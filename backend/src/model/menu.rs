use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
pub struct MenuMast{
    pub _id:i32,
    pub menu_name:String,
    pub menu_link:String,
    pub menu_icon:String,
    pub menu_parent:i32,
}

/*impl MenuMast{
    pub fn default()->Self{
        Self{
            _id:0,
            menu_name:"".to_string(),
            menu_link:"".to_string(),
            menu_icon:"".to_string(),
            menu_parent:0,
        }
    }
}*/