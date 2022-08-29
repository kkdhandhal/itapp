use std::rc::Rc;

use reqwasm::websocket::Message;
use yew::{prelude::*, callback};
use yew_router::components::Link;
use yewdux::{prelude::*, dispatch};
use super::link::MyLink;
use crate::appdata::App_Data;
use crate::route::Route;



pub struct Navbar{
    state:Rc<App_Data>,
    dispatch:Dispatch<App_Data>,
}

pub enum Msg{
    State(Rc<App_Data>),
}


impl Component for Navbar{
    type Message = Msg;
    type Properties = ();

    fn create(ctx:&Context<Self>) -> Self{
        let dispatch = Dispatch::<App_Data>::subscribe(ctx.link().callback(Msg::State));
        Self{
            state:dispatch::get(),
            dispatch,
        }
    }

    fn update(&mut self,_ctx:&Context<Self>,msg: Self::Message) -> bool{
        match msg{
            Msg::State(state) =>{
                    self.state = state;
                    true
            }
        }
    }
    fn view(&self,ctx:&Context<Self>) -> Html{
        //let (app_data, _) = ctx.link().context::<App_Data>(Callback::noop()).unwrap();  //expect("context to be set");
        let user_id = self.state.user_id;
        let onclick_login = self.dispatch.reduce_mut_callback(|s| s.user_id = 13857);
        html!{
            <header class="text-gray-600 body-font bg-gray-900 ">
                                <div class="container mx-auto flex flex-wrap p-2 flex-col md:flex-row items-center">
                                    <a class="flex title-font font-medium items-center text-white-900 mb-4 md:mb-0">
                                        <span class="ml-3 text-xl">{"IT APP"}{user_id}</span>
                                    </a>
                                    <nav class="md:ml-auto flex flex-wrap items-center text-base justify-center">
                                        <a class="mr-5 hover:text-gray-900" href="/login">{"First Link"}</a>
                                        <a class="mr-5 hover:text-gray-900">{"Second Link"}</a>
                                        <a class="mr-5 hover:text-gray-900">{"Third Link"}</a>
                                        <a class="mr-5 hover:text-gray-900">{"Fourth Link"}</a>
                                    </nav>
                                   
                                    <MyLink link_route={Route::Login} classes="inline-flex items-center bg-gray-100 border-0 py-1 px-3 focus:outline-none hover:bg-gray-200 rounded text-base mt-4 md:mt-0" link_name="Login"/>
                                    <MyLink link_route={Route::Home} classes="inline-flex items-center bg-gray-100 border-0 py-1 px-3 focus:outline-none hover:bg-gray-200 rounded text-base mt-4 md:mt-0" link_name="Home"/>
                                    <button class="inline-flex items-center bg-gray-100 border-0 py-1 px-3 focus:outline-none hover:bg-gray-200 rounded text-base mt-4 md:mt-0" onclick={onclick_login}>{"Login"}
                                        <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="w-4 h-4 ml-1" viewBox="0 0 24 24">
                                        <path d="M5 12h14M12 5l7 7-7 7"></path>
                                        </svg>
                                    </button>
                                </div>
            </header>
        }
    }
}