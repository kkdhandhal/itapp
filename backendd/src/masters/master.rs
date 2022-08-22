
use actix_web::web;

use super::mst_ofc;



pub fn init(cfg: &mut web::ServiceConfig){
  cfg.service(
     web::scope("/masters")
       // .service(//add_office)
       /*  web::resource("/ofc")
            .route(web::get().to(mst_ofc::ofc_list))
        )*/
        .service(
          web::resource("/addoffice")
               .route(web::post().to(mst_ofc::add_office))
        ) 
  );  
  
}