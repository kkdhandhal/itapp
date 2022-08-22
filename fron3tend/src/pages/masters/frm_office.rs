use yew::prelude::*;

use crate::model::master::OfcMast;

pub enum Msg{
    AddOffice,

}
pub struct FrmOffice{
    ofc : OfcMast,
    refs : Vec<NodeRef>,
}

impl Component for FrmOffice{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { ofc: OfcMast{
                        _id:0,
                        ofc_name:"".to_string(),
                        ofc_ltcode:"".to_string(),
                        ofc_parent:0,
                    } ,
                refs:vec![NodeRef::default(),NodeRef::default(),NodeRef::default(),NodeRef::default()]
                }
    }
    fn update(&mut self,_ctx: &Context<Self>,msg: Self::Message) -> bool{
       match msg{
            Msg::AddOffice=>  {
               // let client = reqwest::Client::new();
                
             },
             
           
       }
       true
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link().clone();
        html!{
                    <div class="main">
                        <div><h1>{"Office Master"}</h1></div>
                            <label><b>{"Office ID"} </b>  </label>    
                            <input type="text" name="Uname" id="Uname" placeholder="Username" 
                                ref={&self.refs[0]}
                                value={self.ofc._id.clone().to_string()}
                            />    
                            <br/>{&self.ofc._id.to_string()}<br/>    
                            <label><b>{"Office Name"} </b>  </label>    
                            <input type="text" name="Uname" id="Uname" placeholder="Username" 
                                ref={&self.refs[1]}
                                value={self.ofc.ofc_name.clone().to_string()}
                               
                            />    
                            <br/>{self.ofc.ofc_name.clone().to_string()}<br/>   
                            <label><b>{"Office LT Code"} </b>  </label>    
                            <input type="text" name="Uname" id="Uname" placeholder="Username" 
                                ref={&self.refs[2]}
                                value={self.ofc.ofc_ltcode.clone().to_string()}
                               
                            />    
                            <br/>{self.ofc.ofc_ltcode.clone().to_string()}<br/>  
                            <label><b>{"Select Parent Office"} </b>  </label>    
                            <input type="text" name="Uname" id="Uname" placeholder="Username" 
                                ref={&self.refs[3]}
                                value={self.ofc.ofc_parent.clone().to_string()}
                            />    
                            <br/>{self.ofc.ofc_parent.clone().to_string()}<br/>  
                            <button onclick={link.callback(|_| Msg::AddOffice)}>{"Add Office"} </button>       
                            <br/><br/>    
                            
                    </div>
 
        }
    }

}
