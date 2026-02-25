use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use crate::db::AppState;
use crate::ApiResult;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(graphql_handler))
        .route("/playground", get(graphql_playground))
}

pub async fn graphql_handler(
    State(state): State<AppState>,
    body: String,
) -> ApiResult<String> {
    let schema = erp_graphql::build_schema(state.pool);
    let request = async_graphql::Request::new(body);
    let response = schema.execute(request).await;
    Ok(serde_json::to_string(&response)?)
}

pub async fn graphql_playground() -> ApiResult<String> {
    Ok(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <title>GraphQL Playground</title>
    <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
    <link rel="shortcut icon" href="//cdn.jsdelivr.net/npm/graphql-playground-react/build/favicon.png" />
    <script src="//cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
</head>
<body>
    <div id="root">
        <style>
            body { background-color: rgb(23, 42, 58); font-family: "Open Sans", sans-serif; height: 90vh; }
            #root { height: 100%; width: 100%; display: flex; align-items: center; justify-content: center; }
            .loading { font-size: 32px; font-weight: 200; color: rgba(255, 255, 255, .6); margin-left: 20px; }
            img { height: 46px; width: 46px; }
        </style>
        <img src='//cdn.jsdelivr.net/npm/graphql-playground-react/build/logo.png' alt='Logo'>
        <div class="loading"> Loading <span class="loader">...</span> </div>
    </div>
    <script>window.addEventListener('load', function (event) {
        GraphQLPlayground.init(document.getElementById('root'), {
            endpoint: '/api/v1/graphql'
        });
    })</script>
</body>
</html>
    "#.to_string())
}
