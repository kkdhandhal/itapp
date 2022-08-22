
use actix_web::web;

use super::menu;

pub fn init(cfg: &mut web::ServiceConfig){
  cfg.service(
     web::scope("/menu")
       // .service(//add_office)
       /*  web::resource("/ofc")
            .route(web::get().to(mst_ofc::ofc_list))
        )*/
        .service(
          web::resource("/getmenulist/{userid}/{menu_id}")
               .route(web::get().to(menu::get_menu))
        )
  );  
  
}