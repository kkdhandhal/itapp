use yew::{function_component, html, Properties,use_state, use_effect_with_deps, UseStateHandle, Html};
use reqwasm::http::Request;

use crate::model::menu::MenuMast;
use super::icons::Get_Icon;

#[derive(Properties, PartialEq)]
pub struct RenderedAtProps {
    pub userID: String,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &RenderedAtProps) -> Html {

    let get_error = use_state(|| "".to_string());
    
    let get_init_menu:UseStateHandle<Vec<MenuMast>> = use_state(||vec![]);
   {
        let get_error = get_error.clone();
        let get_init_menu = get_init_menu.clone();
       
        use_effect_with_deps(move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let get_error = get_error.clone();
                    let get_init_menu = get_init_menu.clone();
                
                    let fetch_menu = Request::get("http://localhost:8090/menu/getmenulist/0/0")
                                                    .send()
                                                    .await;
                                                    
                    match (fetch_menu) {
                        Ok(fetch_menu) =>{ 
                            match(fetch_menu.json().await){
                                Ok(fetch_menu_list) => get_init_menu.set(fetch_menu_list),
                                Err(e) => get_error.set(e.to_string())
                            }
                        },
                        Err(e) => get_error.set(e.to_string())
                    }
                                                    
                });
         ||()
            },(),
        );
    };
    let mnu_list = (*get_init_menu).clone();
    let inti_menu:Html = mnu_list.into_iter().map(|menu:MenuMast| html!{
        <li class="min-w-max">
            <a href="#" aria-label="dashboard" class="bg group flex items-center space-x-4 rounded-tl-full rounded-bl-full px-4 py-3 text-gray-600 hover:relative hover:flex hover:items-center hover:space-x-4 hover:bg-gradient-to-r hover:from-sky-600 hover:to-cyan-400 hover:px-4 hover:py-3 hover:text-white">
                //<svg class="-ml-1 h-6 w-6" viewBox="0 0 24 24" fill="none">
                <svg id="Layer_1" data-name="Layer 1" viewBox="0 0 24 24" fill="none" class="-ml-1 h-4 w-4">
                {
                    Get_Icon(menu.menu_icon.to_string())
                }
                </svg>
                //<path d="M6 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2V8ZM6 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2v-1Z" class="fill-current text-cyan-400 dark:fill-slate-600"></path>
                //<path d="M13 8a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2V8Z" class="fill-current text-cyan-200 group-hover:text-cyan-300"></path>
                //<path d="M13 15a2 2 0 0 1 2-2h1a2 2 0 0 1 2 2v1a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-1Z" class="fill-current group-hover:text-sky-300"></path>
                //</svg>
                <span class="-mr-1 font-medium">{menu.menu_name}</span>
            </a>
        </li>
    }).collect();//self.value_list.iter().map(|value| html!{<li>{value}</li>});*/

    html! {
        <sidebar class="min-h-screen " >//onmouseenter={on_get_init_menu}>
        <div class="absolute sidebar min-h-screen w-[3rem] overflow-hidden border-r hover:absolute hover:w-2/4 hover:bg-gray-900 hover:shadow-lg">
            <div class="flex h-screen flex-col justify-between pt-2 pb-6">
                <div>
                    <div class="flex w-max p-2.5">
                        <svg id="Layer_1" data-name="Layer 1" viewBox="0 0 24 24" fill="none" class="-ml-1 h-6 w-6">
                            <path d="M18.901,14.709v-4.405c0-0.195-0.15-0.3-0.3-0.195l-4.503,2.597c-0.098,0.053-0.195,0.203-0.195,0.353v5.201  c0,0.203,0.15,0.3,0.3,0.203L18,16.263l3.707,2.146l-9.659,5.576L12.026,24l-0.03-0.015L5.46,20.203L2.36,18.402l3.685-2.139  l3.805,2.199c0.15,0.098,0.3-0.052,0.248-0.248v-5.208c0-0.15-0.052-0.248-0.203-0.345l-4.503-2.604  c-0.098-0.053-0.3,0.052-0.3,0.203v4.398l-3.7,2.154V5.606L11.148,0v4.3L7.343,6.507c-0.15,0.045-0.15,0.3,0,0.345l4.503,2.604  c0.105,0.053,0.255,0.098,0.405,0l4.503-2.604c0.15-0.045,0.15-0.248,0-0.345L12.949,4.3V0l9.659,5.606v11.182L18.901,14.709z" class="fill-current text-cyan-200 group-hover:text-cyan-300"/>
                        </svg>
                        <span class="ml-6 fill-current text-cyan-200 group-hover:text-cyan-300">
                            {"IT APP"}
                        </span>
                    </div>
                    <ul class="mt-6 space-y-2 tracking-wide">
                        {inti_menu}
                    </ul>
                </div>
            </div>
        </div>
        </sidebar>
    }
}

/* */