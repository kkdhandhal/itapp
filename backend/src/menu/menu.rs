
use crate::model::menu::MenuMast;


use mongodb::{bson::doc,Client};
use actix_web::{HttpResponse, web::{self, Json}, http::header::ContentType};

//#[post("addoffice")]
pub async fn get_menu(client : web::Data<Client>,path: web::Path<(i32, i32)>) -> HttpResponse{
    let (userid, menu_id) = path.into_inner();
    let col_menu = client.database("ITAPP").collection::<MenuMast>("mst_menu");

    let cursor = col_menu.find(doc!{"menu_parent":menu_id},None).await;
    match cursor{
        Ok(mut res)=>{
            let mut menu_list:Vec<MenuMast> =Vec::new();
            
            while res.advance().await.unwrap(){
                menu_list.push(res.deserialize_current().unwrap());
            }
            let js = serde_json::to_string(&menu_list).unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(js)
        } ,
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}