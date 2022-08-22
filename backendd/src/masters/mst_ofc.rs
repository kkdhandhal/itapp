use crate::model::master::OfcMast;


use mongodb::{Client};
use actix_web::{HttpResponse, web};

//#[post("addoffice")]
pub async fn add_office(client : web::Data<Client>,ofc:web::Json<OfcMast>) -> HttpResponse{
    
    let col_office = client.database("ITAPP").collection::<OfcMast>("mst_office");
    let result = col_office.insert_one(ofc.clone(), None).await;
    match result{
        Ok(res)=> HttpResponse::Ok().body(format!("{} - {} office is added sucessfuly with ID {}",ofc._id,ofc.ofc_name,res.inserted_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}