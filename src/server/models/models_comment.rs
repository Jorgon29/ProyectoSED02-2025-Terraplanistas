use uuid::Uuid;
pub struct Comment{
    id: Uuid,
    author: Uuid,
    writing: Uuid,
    content: String
}