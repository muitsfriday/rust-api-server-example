use chrono::Utc;
use std::error::Error;

pub struct Service {}

use super::Post;

impl Service {
    pub fn new() -> Self {
        Service {}
    }

    pub fn get(&self, id: &str) -> Result<Post, Box<dyn Error>> {
        let mut post = Post::default();
        post.id = String::from(id);

        Ok(post)
    }

    pub fn create(&self, data: Post) -> Result<Post, Box<dyn Error>> {
        let now = Utc::now();
        let post = Post {
            id: String::from("mockup_id"),
            created_at: Some(now),
            updated_at: Some(now),
            deleted_at: None,
            ..data
        };

        Ok(post)
    }
}
