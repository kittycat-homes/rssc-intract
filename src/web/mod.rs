use rocket::Route;
mod about;
mod user;

pub fn routes() -> Vec<Route> {
    routes![user::profile, about::about]
}
