use std::env;
use std::io;
use std::sync::Arc;

use actix_web::error::BlockingError;
use actix_web::web::{block, get, post, resource, Json};
use actix_web::{middleware, web::Data, App, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::{GraphQLRequest, GraphQLResponse};
use juniper::DefaultScalarValue;
use serde_json::error::Error as SerdeError;

// use repository::mongodb::establish_mongodb_connection;
use repository::memory::status::post::PostRepository;
use server::http::graphql::schema::{schema, MutationRoot, QueryRoot, Schema};
use status::post::PostService;

async fn graphiql() -> HttpResponse {
    let html: String = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: Data<Arc<Schema<PostService<PostRepository>>>>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, BlockingError<SerdeError>> {
    let user: String = block(move || {
        let res: GraphQLResponse<DefaultScalarValue> = data.execute(&st, &());
        Ok::<_, SerdeError>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // let collection = establish_mongodb_connection("stoodly", "post");

    env::set_var("RUST_LOG", "info");
    env_logger::init();
    HttpServer::new(move || {
        let query_root = QueryRoot {
            post_service: PostService {
                repository: PostRepository {
                    // collection: collection.clone(),
                },
            },
        };
        let mutation_root = MutationRoot {
            post_service: PostService {
                repository: PostRepository {
                    // collection: collection.clone(),
                },
            },
        };
        App::new()
            .data(Arc::new(schema(query_root, mutation_root)))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(resource("/graphql").route(post().to(graphql)))
            .service(resource("/graphiql").route(get().to(graphiql)))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
