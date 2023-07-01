use rocket::Route;

pub type RouteMapping = (&'static str, Vec<Route>);
