use uuid::Uuid;
use crate::server::models::{models_comment::Comment, models_tag::Tag};

pub struct WritingDisplay{
    pub id: Uuid,
    pub title: String,
    pub image: String,
    pub tags: Vec<Tag>,
    pub writing_type: Uuid,
    pub author: Uuid
}

pub struct WritingFull{
    pub id: Uuid,
    pub title: String,
    pub image: String,
    pub tags: Vec<Tag>,
    pub writing_type: Uuid,
    pub author: Uuid,
    pub content: String,
    pub comments: Vec<Comment>
}