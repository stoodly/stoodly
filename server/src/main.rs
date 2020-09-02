use std::env;
use std::io;

use actix_cors::Cors;
use actix_web::{App, Error, HttpResponse, HttpServer, middleware, web};
use juniper_actix::{graphql_handler, playground_handler, graphiql_handler};

use repository::mongodb::status::post::PostRepository;
use repository::mongodb::establish_mongodb_connection;
use server::http::graphql::schema::{MutationRoot, QueryRoot, schema, Schema};
use status::post::PostService;

async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/", None).await
}

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/", None).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema<PostService<PostRepository>>>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &(), req, payload).await
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let collection = establish_mongodb_connection("stoodly", "post").expect("expected 'post' collection in the 'stoodly' db");

    env::set_var("RUST_LOG", "info");
    env_logger::init();
    HttpServer::new(move || {
        let query_root = QueryRoot {
            post_service: PostService {
                repository: PostRepository {
                    collection: collection.clone(),
                },
            },
        };
        let mutation_root = MutationRoot {
            post_service: PostService {
                repository: PostRepository {
                    collection: collection.clone(),
                },
            },
        };
        App::new()
            .data(schema(query_root, mutation_root))
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
