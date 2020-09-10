use std::env;
use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use repository::mongodb::account::user::UserRepository;
use repository::mongodb::establish_mongodb_connection;
use repository::mongodb::organization::team::TeamRepository;
use repository::mongodb::status::post::PostRepository;
use server::http::graphql::schema::{schema, MutationRoot, QueryRoot, Schema};
use domain::status::StatusService;
use domain::status::post::PostService;
use domain::account::user::UserService;
use domain::organization::team::TeamService;

async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/", None).await
}

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/", None).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<
        Schema<
            StatusService<
                PostService<PostRepository>,
                UserService<UserRepository>,
                TeamService<TeamRepository>,
            >,
        >,
    >,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &(), req, payload).await
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let post_collection = establish_mongodb_connection("stoodly", "post")
        .expect("expected 'post' collection in the 'stoodly' db");
    let user_collection = establish_mongodb_connection("stoodly", "user")
        .expect("expected 'user' collection in the 'stoodly' db");
    let team_collection = establish_mongodb_connection("stoodly", "team")
        .expect("expected 'team' collection in the 'stoodly' db");
    env::set_var("RUST_LOG", "info");
    env_logger::init();
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
