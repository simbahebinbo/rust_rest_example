#[macro_use]
extern crate rbatis;

use fast_log::config::Config;

use actix_web::{middleware, App, HttpServer};
use actix_web::web::{Data};

use crate::model::{Users};
use crate::service::UsersService;

mod model;
mod service;
mod handlers;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let host = "localhost";
    let port = "5432";
    let user = "pig";
    let password = "123456";
    let dbname = "example";

    let rconn_string = std::format!("postgres://{user}:{password}@{host}:{port}/{dbname}",
                                    host=host, port=port, user=user, password=password, dbname=dbname);
    
    fast_log::init(Config::new().console()).unwrap();
    log::info!("Starting");

    let data: Data<UsersService> = Data::new(UsersService::new(&rconn_string).await);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::list)
            .service(handlers::get_user_by_id)
            .service(handlers::create_user)
            .service(handlers::update_user)
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

