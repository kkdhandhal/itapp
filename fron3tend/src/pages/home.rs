use yew::prelude::*;
use crate::component::{sidebar::Sidebar,navbar::Navbar};

pub struct Home;


impl Component for Home{
    type Message = ();
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self{
        Self{}
    }

    fn update(&mut self,_ctx: &Context<Self>,_msg: Self::Message) -> bool{
       true
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let _link = ctx.link();
        html!{
                <div class="bg-gray-900 h-screen">
                
                    <div class="flex">
                        <Sidebar userID="13857"/>
                        <div class="w-full" >
                           <Navbar/>
                            <container>
                                
                            </container>
                        </div>
                    </div>
                </div>
        }
    }

}
