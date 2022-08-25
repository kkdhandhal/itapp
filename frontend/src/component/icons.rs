use yew::{function_component, html, Properties,use_state, Callback,use_effect_with_deps, UseStateHandle, Html};

pub fn Get_Icon(icon:String)-> Html{
    match icon.as_str(){
        "icn_dashbord" => {
            html!{
               // <svg id="Layer_1" data-name="Layer 1" viewBox="0 0 24 24" fill="none" class="-ml-1 h-4 w-4">
               <>
                    <path d="M23,22H5a3,3,0,0,1-3-3V1A1,1,0,0,0,0,1V19a5.006,5.006,0,0,0,5,5H23a1,1,0,0,0,0-2Z" class="fill-current text-cyan-200 group-hover:text-orange-300"/><path d="M6,20a1,1,0,0,0,1-1V12a1,1,0,0,0-2,0v7A1,1,0,0,0,6,20Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                    <path d="M10,10v9a1,1,0,0,0,2,0V10a1,1,0,0,0-2,0Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                    <path d="M15,13v6a1,1,0,0,0,2,0V13a1,1,0,0,0-2,0Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                    <path d="M20,9V19a1,1,0,0,0,2,0V9a1,1,0,0,0-2,0Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                    <path d="M6,9a1,1,0,0,0,.707-.293l3.586-3.586a1.025,1.025,0,0,1,1.414,0l2.172,2.172a3,3,0,0,0,4.242,0l5.586-5.586A1,1,0,0,0,22.293.293L16.707,5.878a1,1,0,0,1-1.414,0L13.121,3.707a3,3,0,0,0-4.242,0L5.293,7.293A1,1,0,0,0,6,9Z" class="fill-current text-lime-400 group-hover:text-orange-300"/>
                </>
              //  </svg>
            }
        },
        "icn_module" => {
            html!{
                <path d="M0,19H5v5H3c-1.657,0-3-1.343-3-3v-2ZM5,5V0H3C1.343,0,0,1.343,0,3v2H5Zm7,
                0V3c0-1.657-1.343-3-3-3h-2V5h5ZM0,7v10H5V7H0Zm7,0v10h5V7H7Zm0,17h2c1.657,0,3-1.343,3-3v-2H7v5ZM13.424,
                7.478l3.639,10.944,5.412-1.795-3.639-10.944-5.412,1.795Zm4.27,12.841l.792,2.312c.348,1.048,1.48,1.615,2.528,
                1.267l1.615-.535c1.049-.348,1.617-1.481,1.268-2.529l-.791-2.309-5.412,1.795ZM12.793,
                5.58l5.412-1.795-.803-2.415c-.348-1.048-1.48-1.616-2.528-1.268l-1.615,.535c-1.048,.348-1.616,1.48-1.268,
                2.528l.803,2.415Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
            }
        },
        "icn_setting" =>{
            html!{
                <>
                    <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 
                    0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 
                    2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 
                    2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 
                    2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 
                    01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 
                    3 0 000 6z" clip-rule="evenodd" class="fill-current text-cyan-300 group-hover:text-orange-300 text-lg"/>
                </>
            }
        }
        _ =>  html!{
           // <svg id="Layer_1" data-name="Layer 1" viewBox="0 0 24 24" fill="none" class="-ml-1 h-4 w-4">
           <>
                <path d="M23,22H5a3,3,0,0,1-3-3V1A1,1,0,0,0,0,1V19a5.006,5.006,0,0,0,5,5H23a1,1,0,0,0,0-2Z" class="fill-current text-cyan-200 group-hover:text-orange-300"/><path d="M6,20a1,1,0,0,0,1-1V12a1,1,0,0,0-2,0v7A1,1,0,0,0,6,20Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                <path d="M10,10v9a1,1,0,0,0,2,0V10a1,1,0,0,0-2,0Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                <path d="M15,13v6a1,1,0,0,0,2,0V13a1,1,0,0,0-2,0Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                <path d="M20,9V19a1,1,0,0,0,2,0V9a1,1,0,0,0-2,0Z" class="fill-current text-cyan-300 group-hover:text-orange-300"/>
                <path d="M6,9a1,1,0,0,0,.707-.293l3.586-3.586a1.025,1.025,0,0,1,1.414,0l2.172,2.172a3,3,0,0,0,4.242,0l5.586-5.586A1,1,0,0,0,22.293.293L16.707,5.878a1,1,0,0,1-1.414,0L13.121,3.707a3,3,0,0,0-4.242,0L5.293,7.293A1,1,0,0,0,6,9Z" class="fill-current text-lime-400 group-hover:text-orange-300"/>
            </>
          //  </svg>
        }
    }
    
} 
