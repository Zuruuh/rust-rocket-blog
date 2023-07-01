mod blog_controller;
mod mapping;

pub fn routes() -> Vec<mapping::RouteMapping> {
    vec![blog_controller::routes()]
}
