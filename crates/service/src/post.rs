use crate::error::ServiceError;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use model::post::Post;
use mongodb::bson::doc;
use mongodb::Collection;

#[async_trait]
pub trait PostService {
    async fn new(&self, post: Post) -> Result<(), ServiceError>;
    async fn get(&self, id: String) -> Result<Post, ServiceError>;
    async fn get_all(&self) -> Result<Vec<Post>, ServiceError>;
    async fn delete(&self, id: String) -> Result<(), ServiceError>;
}

#[derive(Debug, Clone)]
pub struct PostServiceImpl {
    pub collection: Collection<Post>,
}

#[async_trait]
impl PostService for PostServiceImpl {
    async fn new(&self, post: Post) -> Result<(), ServiceError> {
        self.collection.insert_one(post, None).await?;
        Ok(())
    }
    async fn get(&self, id: String) -> Result<Post, ServiceError> {
        self.collection
            .find_one(doc! {"name": id}, None)
            .await?
            .ok_or(ServiceError::NotFound)
    }
    async fn get_all(&self) -> Result<Vec<Post>, ServiceError> {
        let cursor = self.collection.find(None, None).await?;
        cursor.try_collect().await.map_err(Into::into)
    }
    async fn delete(&self, id: String) -> Result<(), ServiceError> {
        self.collection
            .delete_one(doc! {"name": id}, None)
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}
