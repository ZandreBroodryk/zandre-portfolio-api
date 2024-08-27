use axum::{
    extract::{Path, State},
    middleware,
    routing::{get, post},
    Json, Router,
};

use crate::{
    context::{middleware_ctx_resolver, Context},
    model::{
        blog::{BlogController, BlogSummary, DetailedBlog, NewBlog},
        ApiState,
    },
    Result,
};

pub fn routes(state: ApiState) -> Router {
    Router::new()
        .route("/", post(create_blog_post).get(get_blog_posts))
        .route("/:id", get(get_detailed_blog))
        .route_layer(middleware::from_fn(middleware_ctx_resolver))
        .with_state(state)
}

async fn create_blog_post(
    State(blog_controller): State<BlogController>,
    context: Context,
    Json(blog): Json<NewBlog>,
) -> Result<Json<BlogSummary>> {
    let blog_summary = blog_controller.create_blog(context, blog).await?;
    return Ok(Json(blog_summary));
}

async fn get_blog_posts(
    State(blog_controller): State<BlogController>,
) -> Result<Json<Vec<BlogSummary>>> {
    let blog_summaries = blog_controller.get_blog_posts().await?;
    return Ok(Json(blog_summaries));
}

async fn get_detailed_blog(
    State(blog_controller): State<BlogController>,
    Path(id): Path<i32>,
) -> Result<Json<DetailedBlog>> {
    let detailed_blog = blog_controller.get_detailed_blog(id).await?;
    return Ok(Json(detailed_blog));
}
