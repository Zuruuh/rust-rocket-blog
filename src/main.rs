use dotenv::dotenv;
use rocket_db_pools::Database;

mod db;
mod controllers;

#[rocket::launch]
fn rocket() -> _ {
    dotenv().ok();

    let routes = controllers::routes();
    let mut app = rocket::build()
        .attach(db::Blog::init())
    ;

    for route in routes {
        app = app.mount(route.0, route.1);
    }

    app
}
