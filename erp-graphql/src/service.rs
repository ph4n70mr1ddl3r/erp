use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use sqlx::SqlitePool;

pub use crate::schema::*;

pub type GraphQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(pool: SqlitePool) -> GraphQLSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}
