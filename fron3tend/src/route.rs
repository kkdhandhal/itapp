use crate::pages::{home::Home,masters::frm_office};
use yew::prelude::*;
use yew_router::{prelude::*};


#[derive(Clone,Routable,PartialEq)]
pub enum Route{
    #[at("/")]
    Home,
   // #[at("/master")]
    //Login,
    #[not_found]
    #[at("/404")]
    NotFound,
    
}

pub fn switch(routes :&Route) -> Html{
    match routes{
        Route::Home => html!{<Home/>},
        //Route::Login => html!{<frm_office::FrmOffice/>},
        Route::NotFound =>html!{<h1>{"Page Not Found"}</h1>},
    }
}