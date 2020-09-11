use juniper::{EmptySubscription, FieldResult};
use juniper::RootNode;
use uuid::Uuid;

use domain::status::post::Post;
use domain::status::Service;

use crate::status::request::{NewPost, QueryPost, UpdatePost};

pub struct QueryRoot<P: Service> {
    pub status_service: P,
}

#[juniper::graphql_object]
impl<P: Service> QueryRoot<P> {
    fn post(&self, id: Uuid) -> FieldResult<Option<QueryPost>> {
        fn mk_query_post(post: Option<Post>) -> Option<QueryPost> {
            match post {
                Some(value) => Some(QueryPost::from_post(value)),
                None => None,
            }
        }
        match self.status_service.read(id, Uuid::new_v4()) {
            Ok(post_opt) => Ok(mk_query_post(post_opt)),
            Err(error) => Err(error)?,
        }
    }
}

pub struct MutationRoot<P: Service> {
    pub status_service: P,
}

#[juniper::graphql_object]
impl<P: Service> MutationRoot<P> {
    fn create_post(&self, new_post: NewPost) -> FieldResult<QueryPost> {
        match self
            .status_service
            .create(NewPost::from_new_post(&new_post), Uuid::new_v4())
        {
            Ok(post) => Ok(QueryPost::from_post(post)),
            Err(error) => Err(error)?,
        }
    }

    fn update_post(&self, update_post: UpdatePost) -> FieldResult<QueryPost> {
        match self
            .status_service
            .update(UpdatePost::from_update_post(&update_post), Uuid::new_v4())
        {
            Ok(post) => Ok(QueryPost::from_post(post)),
            Err(error) => Err(error)?,
        }
    }

    fn delete_post(&self, id: Uuid) -> FieldResult<Option<QueryPost>> {
        fn mk_query_post(post: Option<Post>) -> Option<QueryPost> {
            match post {
                Some(value) => Some(QueryPost::from_post(value)),
                None => None,
            }
        }
        match self.status_service.delete(id, Uuid::new_v4()) {
            Ok(post_opt) => Ok(mk_query_post(post_opt)),
            Err(error) => Err(error)?,
        }
    }
}

pub type Schema<P> = RootNode<'static, QueryRoot<P>, MutationRoot<P>, EmptySubscription>;

pub fn schema<P: Service>(query: QueryRoot<P>, mutation: MutationRoot<P>) -> Schema<P> {
    Schema::new(query, mutation, EmptySubscription::new())
}
