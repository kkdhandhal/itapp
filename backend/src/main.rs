mod masters;
mod error;
mod model;
mod menu;
use std::path::PathBuf;

//use crate::user::route;
use crate::masters::master;
use crate::menu::init;
use actix_files::NamedFile;
use actix_web::{get,web,App,HttpServer,Result,http,HttpRequest};
use actix_cors::Cors;
use mongodb::{Client,Collection};


/* async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./dist/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = format!("./dist/{}",req.match_info().query("filename")).parse().unwrap();
    Ok(NamedFile::open(path)?)
} */

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await.expect("Fail to Connect");
    
    HttpServer::new(move||{
        let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header();
            /* .allowed_origin_fn(|origin,_req_header|{
                origin.as_bytes().starts_with(b"http://localhost")
            }); */
            
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(cors)
            .configure(masters::master::init)
            .configure(menu::init::init)
           // .route("/",web::get().to(index))
          //  .route("/{filename:.*}", web::get().to(static_file))
           // .configure(user::route::init)
    })
    .bind(("127.0.0.1",8090))?
    .run()
    .await

}
