use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{context::Context, Result, ServerError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogSummary {
    id: i32,
    title: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedBlog {
    created_at: NaiveDate,
    author: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogWriteModel {
    content: String,
}

#[derive(Clone)]
pub struct BlogController {
    pool: PgPool,
}

impl BlogController {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl BlogController {
    pub async fn create_blog(&self, context: Context, blog: BlogWriteModel) -> Result<BlogSummary> {
        let title = blog
            .content
            .split('\n')
            .collect::<Vec<&str>>()
            .first()
            .ok_or(ServerError::NoTitleForBlogPost)?
            .trim_start_matches("# ")
            .to_string();

        let content = blog.content;

        let id = sqlx::query_file!(
            "src/model/sql/add_blog_post.sql",
            title,
            content,
            context.current_user_id()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|error| ServerError::DatabaseFailure(error.to_string()))?
        .id;

        return Ok(BlogSummary {
            id: id,
            title: title,
        });
    }

    pub async fn update_blog_post(
        &self,
        context: Context,
        id: i32,
        blog: BlogWriteModel,
    ) -> Result<BlogSummary> {
        let title = blog
            .content
            .split('\n')
            .collect::<Vec<&str>>()
            .first()
            .ok_or(ServerError::NoTitleForBlogPost)?
            .trim_start_matches("# ")
            .to_string();

        let content = blog.content;

        sqlx::query_file!(
            "src/model/sql/update_blog_post.sql",
            id,
            title,
            content,
            context.current_user_id()
        )
        .execute(&self.pool)
        .await
        .map_err(|error| ServerError::DatabaseFailure(error.to_string()))?;

        return Ok(BlogSummary {
            id: id,
            title: title,
        });
    }

    pub async fn get_blog_posts(&self) -> Result<Vec<BlogSummary>> {
        let blog_post_summaries =
            sqlx::query_file_as!(BlogSummary, "src/model/sql/get_blog_posts.sql")
                .fetch_all(&self.pool)
                .await
                .map_err(|error| ServerError::DatabaseFailure(error.to_string()))?;
        return Ok(blog_post_summaries);
    }

    pub async fn get_detailed_blog(&self, id: i32) -> Result<DetailedBlog> {
        let detailed_blog =
            sqlx::query_file_as!(DetailedBlog, "src/model/sql/get_detailed_blog_post.sql", id)
                .fetch_one(&self.pool)
                .await
                .map_err(|error| ServerError::DatabaseFailure(error.to_string()))?;

        return Ok(detailed_blog);
    }
}
