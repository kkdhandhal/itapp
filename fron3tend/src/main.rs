mod route;
mod pages;
mod component;
mod model;

use yew::prelude::*;
use yew_router::{Switch,BrowserRouter};
use route::{Route,switch};

// enum Msg{
//     Addone,
// }

struct Model;

impl Component for Model{
    type Message = ();
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self{
       Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html{
        //let link = ctx.link();
        html!{
             <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
            
               
        }
    }

}

fn main(){
    yew::start_app::<Model>();
}