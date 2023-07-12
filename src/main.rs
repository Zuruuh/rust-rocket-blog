use dotenv::dotenv;

mod schema;
mod controllers;
mod db;
mod dto;
mod models;
mod repositories;

#[rocket::launch]
fn rocket() -> _ {
    dotenv().ok();

    let routes = controllers::routes();

    let mut app = rocket::build()
        .attach(db::BlogConnection::fairing())
    ;

    for (prefix, subroutes) in routes {
        app = app.mount(prefix, subroutes);
    }

    app
}
