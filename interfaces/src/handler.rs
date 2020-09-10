use actix_web::{Error, HttpResponse, web};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use application::account::user::UserService;
use application::organization::team::TeamService;
use application::status::post::PostService;
use application::status::StatusService;
use infrastructure::mongodb::account::user::UserRepository;
use infrastructure::mongodb::organization::team::TeamRepository;
use infrastructure::mongodb::status::post::PostRepository;

use crate::schema::Schema;

pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/", None).await
}

pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/", None).await
}

pub async fn graphql(
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