use uuid::Uuid; 

#[derive(Debug)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
}