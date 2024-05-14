use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod db;
mod schema;
mod endpoints;
mod validations;

use endpoints::{
    admin::{get_admin, post_admin}, routine::{get_student_routine_by_day, get_routine, post_routine}
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::test_connection().await.expect("Error connecting to database");
    // db::db("movies").await.expect("Error fetching data");

    // Start the HTTP server and bind it to the specified address
    HttpServer::new(|| {
        // Create an App instance
        App::new()
            // Register a route and a handler
            .service(get_routine)
            .service(post_routine)
            .service(get_admin)
            .service(post_admin)
            .service(get_student_routine_by_day)
    })
    .bind("127.0.0.1:5555")? // Bind to localhost on port 8080
    .run()
    .await // Start the server
}
