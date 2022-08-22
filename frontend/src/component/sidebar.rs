use yew::{function_component, html, Properties,use_state, Callback, UseStateHandle, Html};
use reqwasm::http::Request;

use crate::model::master::MenuMast;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub userID: String,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &RenderedAtProps) -> Html {

    let Get_Init_Menu:UseStateHandle<Vec<MenuMast>> = use_state(||vec![]);
    let Get_Error = use_state(|| "".to_string());

    let on_get_init_menu={
        let Get_Error = Get_Error.clone();
        let Get_Init_Menu = Get_Init_Menu.clone();
        

        wasm_bindgen_futures::spawn_local(async move {
            let Get_Error = Get_Error.clone();
            let Get_Init_Menu = Get_Init_Menu.clone();
           
            let fetch_menu = Request::get("http://localhost:8090/menu/getmenulist/0/0")
                                            .send()
                                            .await;
                                            
            match (fetch_menu) {
                Ok(fetch_menu) =>{ 
                    match(fetch_menu.json().await){
                        Ok(fetch_menu_list) => Get_Init_Menu.set(fetch_menu_list),
                        Err(e) => Get_Error.set(e.to_string())
                    }
                },
                Err(e) => Get_Error.set(e.to_string())
            }
                                            
        });
        
    };
    let mnu_list = (*Get_Init_Menu).clone();
    let inti_menu:Html = mnu_list.into_iter().map(|menu:MenuMast| html!{
        <li class="min-w-max">
            <a href="#" aria-label="dashboard" class="bg group flex items-center space-x-4 rounded-tl-full rounded-bl-full px-4 py-3 text-gray-600 hover:relative hover:flex hover:items-center hover:space-x-4 hover:bg-gradient-to-r hover:from-sky-600 hover:to-cyan-400 hover:px-4 hover:py-3 hover:text-white">
                <svg class="-ml-1 h-6 w-6" viewBox="0 0 24 24" fill="none">
                <path d="M6 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V8ZM6 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2v-1Z" class="fill-current text-cyan-400 dark:fill-slate-600"></path>
                <path d="M13 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2V8Z" class="fill-current text-cyan-200 group-hover:text-cyan-300"></path>
                <path d="M13 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-1Z" class="fill-current group-hover:text-sky-300"></path>
                </svg>
                <span class="-mr-1 font-medium">{menu.menu_name}</span>
            </a>
        </li>
    }).collect();//self.value_list.iter().map(|value| html!{<li>{value}</li>});*/

    html! {
        <sidebar class="min-h-screen ">
        <div class="absolute sidebar min-h-screen w-[3.35rem] overflow-hidden border-r hover:absolute hover:w-2/4 hover:bg-gray-900 hover:shadow-lg">
            <div class="flex h-screen flex-col justify-between pt-2 pb-6">
                <div>
                    <div class="w-max p-2.5">
                        <img src="https://tailus.io/images/logo.svg" class="w-32" alt="" />
                    </div>
                    <ul class="mt-6 space-y-2 tracking-wide">
                        {inti_menu}
                        /* <li class="min-w-max">
                            <a href="#" aria-label="dashboard" class="bg group flex items-center space-x-4 rounded-tl-full rounded-bl-full px-4 py-3 text-gray-600 hover:relative hover:flex hover:items-center hover:space-x-4 hover:bg-gradient-to-r hover:from-sky-600 hover:to-cyan-400 hover:px-4 hover:py-3 hover:text-white">
                                <svg class="-ml-1 h-6 w-6" viewBox="0 0 24 24" fill="none">
                                <path d="M6 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V8ZM6 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2v-1Z" class="fill-current text-cyan-400 dark:fill-slate-600"></path>
                                <path d="M13 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2V8Z" class="fill-current text-cyan-200 group-hover:text-cyan-300"></path>
                                <path d="M13 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-1Z" class="fill-current group-hover:text-sky-300"></path>
                                </svg>
                                <span class="-mr-1 font-medium">{"Dashboard"}</span>
                            </a>
                        </li>
                         <li class="min-w-max">
                            <a href="#" class="bg group flex items-center space-x-4 rounded-full px-4 py-3 text-gray-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path class="fill-current text-gray-300 group-hover:text-cyan-300" fill-rule="evenodd" d="M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H8a3 3 0 00-3 3v1.5a1.5 1.5 0 01-3 0V6z" clip-rule="evenodd" />
                                <path class="fill-current text-gray-600 group-hover:text-cyan-600" d="M6 12a2 2 0 012-2h8a2 2 0 012 2v2a2 2 0 01-2 2H2h2a2 2 0 002-2v-2z" />
                                </svg>
                                <span class="group-hover:text-gray-700">{"Categories"}</span>
                            </a>
                        </li>
                        <li class="min-w-max">
                            <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path class="fill-current text-gray-600 group-hover:text-cyan-600" fill-rule="evenodd" d="M2 5a2 2 0 012-2h8a2 2 0 012 2v10a2 2 0 002 2H4a2 2 0 01-2-2V5zm3 1h6v4H5V6zm6 6H5v2h6v-2z" clip-rule="evenodd" />
                                <path class="fill-current text-gray-300 group-hover:text-cyan-300" d="M15 7h1a2 2 0 012 2v5.5a1.5 1.5 0 01-3 0V7z" />
                                </svg>
                                <span class="group-hover:text-gray-700">{"Reports"}</span>
                            </a>
                        </li>
                        <li class="min-w-max">
                            <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path class="fill-current text-gray-600 group-hover:text-cyan-600" d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z" />
                                <path class="fill-current text-gray-300 group-hover:text-cyan-300" d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z" />
                                </svg>
                                <span class="group-hover:text-gray-700">{"Other data"}</span>
                            </a>
                        </li>
                        <li class="min-w-max">
                            <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path class="fill-current text-gray-300 group-hover:text-cyan-300" d="M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4z" />
                                <path class="fill-current text-gray-600 group-hover:text-cyan-600" fill-rule="evenodd" d="M18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z" clip-rule="evenodd" />
                                </svg>
                                <span class="group-hover:text-gray-700">{"Finance"}</span>
                            </a>
                        </li> */
                    </ul>
                </div>
                <div class="w-max -mb-3">
                    <a href="#" class="group flex items-center space-x-4 rounded-md px-4 py-3 text-gray-600">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 group-hover:fill-cyan-600" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
                        </svg>
                        <span class="group-hover:text-gray-700">{"Settings"}</span>
                    </a>
                </div>
            </div>
        </div>
        </sidebar>
    }
}

/* */