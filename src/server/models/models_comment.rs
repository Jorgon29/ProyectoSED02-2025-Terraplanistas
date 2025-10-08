use uuid::Uuid;
pub struct Comment{
    pub id: Uuid,
    pub author: String,
    pub content: String
}