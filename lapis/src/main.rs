// Project Lapis
//
// crate:lapis - project initializer. handles all configuration and pre-init logic
// crate:lapis-api - handles the api & back-end. communicates with database
// crate:lapis-yew - yew front-end
//

use lapis_api;
use lapis_yew;

use dotenv::dotenv;
use log::{error, info};

fn main() {
    env_logger::init();
    info!("Initializing...");

    info!("Checking .env");
    dotenv().ok();

    info!("Starting the API");
    lapis_api::init(lapis_api::ApiStartupParams {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set."),
        jwt_expired_in: std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set."),
        jwt_max_age: std::env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set."),
    });
}
