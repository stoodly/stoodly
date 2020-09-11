#[macro_use]
extern crate log;
use log::Level;
use std::{env, io};

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

use application::account::user::UserService;
use application::organization::team::TeamService;
use application::status::post::PostService;
use application::status::StatusService;
use infrastructure::mongodb::account::user::UserRepository;
use infrastructure::mongodb::establish_mongodb_connection;
use infrastructure::mongodb::organization::team::TeamRepository;
use infrastructure::mongodb::status::post::PostRepository;
use interfaces::handler::{graphiql, graphql, playground};
use interfaces::schema::{schema, MutationRoot, QueryRoot};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    if log_enabled!(Level::Debug) {
        debug!("Environment Variables:");
        for (key, value) in env::vars() {
            debug!("{}: {}", key, value);
        }
    }

    let post_collection = establish_mongodb_connection(
        "stoodly",
        "post",
        env::var("MONGODB_URL")
            .expect("expected MONGODB_URL")
            .as_str(),
    )
    .expect("expected 'post' collection in the 'stoodly' db");
    let user_collection = establish_mongodb_connection(
        "stoodly",
        "user",
        env::var("MONGODB_URL")
            .expect("expected MONGODB_URL")
            .as_str(),
    )
    .expect("expected 'user' collection in the 'stoodly' db");
    let team_collection = establish_mongodb_connection(
        "stoodly",
        "team",
        env::var("MONGODB_URL")
            .expect("expected MONGODB_URL")
            .as_str(),
    )
    .expect("expected 'team' collection in the 'stoodly' db");

    HttpServer::new(move || {
        App::new()
            .data(schema(
                QueryRoot {
                    status_service: StatusService {
                        post_service: PostService {
                            repository: PostRepository {
                                collection: post_collection.clone(),
                            },
                        },
                        user_service: UserService {
                            repository: UserRepository {
                                collection: user_collection.clone(),
                            },
                        },
                        team_service: TeamService {
                            repository: TeamRepository {
                                collection: team_collection.clone(),
                            },
                        },
                    },
                },
                MutationRoot {
                    status_service: StatusService {
                        post_service: PostService {
                            repository: PostRepository {
                                collection: post_collection.clone(),
                            },
                        },
                        user_service: UserService {
                            repository: UserRepository {
                                collection: user_collection.clone(),
                            },
                        },
                        team_service: TeamService {
                            repository: TeamRepository {
                                collection: team_collection.clone(),
                            },
                        },
                    },
                },
            ))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
