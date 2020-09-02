use juniper::RootNode;
use juniper::{EmptySubscription, FieldResult};
use uuid::Uuid;

use status::post::{Post, Service};

use crate::http::graphql::status::{NewPost, QueryPost, UpdatePost};

pub struct QueryRoot<P: Service> {
    pub post_service: P,
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
        match self.post_service.read(id) {
            Ok(post_opt) => Ok(mk_query_post(post_opt)),
            Err(error) => Err(error)?,
        }
    }
}

pub struct MutationRoot<P: Service> {
    pub post_service: P,
}

#[juniper::graphql_object]
impl<P: Service> MutationRoot<P> {
    fn create_post(&self, new_post: NewPost) -> FieldResult<QueryPost> {
        match self.post_service.create(NewPost::from_new_post(&new_post)) {
            Ok(post) => Ok(QueryPost::from_post(post)),
            Err(error) => Err(error)?,
        }
    }

    fn update_post(&self, update_post: UpdatePost) -> FieldResult<QueryPost> {
        match self
            .post_service
            .update(UpdatePost::from_update_post(&update_post))
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
        match self.post_service.delete(id) {
            Ok(post_opt) => Ok(mk_query_post(post_opt)),
            Err(error) => Err(error)?,
        }
    }
}

pub type Schema<P> = RootNode<'static, QueryRoot<P>, MutationRoot<P>, EmptySubscription>;

pub fn schema<P: Service>(query: QueryRoot<P>, mutation: MutationRoot<P>) -> Schema<P> {
    Schema::new(query, mutation, EmptySubscription::new())
}
