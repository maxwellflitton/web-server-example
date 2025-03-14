pub mod users;
pub mod auth;
pub mod roles;
use actix_web::web::ServiceConfig;


pub fn views_factory(app: &mut ServiceConfig) {
    users::users_factory(app);
    auth::auth_factory(app);
    roles::roles_factory(app);
}
