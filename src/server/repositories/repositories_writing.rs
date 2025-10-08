use postgres::Error;
use uuid::Uuid;
use crate::server::models::models_comment::Comment;
use crate::server::models::models_writing::{WritingFull, WritingDisplay};
use crate::server::models::models_tag::Tag;

pub fn create_writing(author: Uuid, title: String, content: String, tags: Vec<Uuid>, cover: String, writing_type: Uuid) -> Result<u64, Error>{
    let mut client = crate::server::db::db_connect::get_database_client()?;
    let mut transaction = client.transaction()?;

    let writing_insert_query = "
        INSERT INTO schema_seguridad.WRITING (title, content, cover, id_type, id_user)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
    ";

    let tag_insert_query = "
        INSERT INTO schema_seguridad.WRITINGXTAG (id_tag, id_writing)
        VALUES ($1, $2)
    ";

    let new_writing_id: Uuid = transaction.query_one(
        writing_insert_query,
        &[&title, &content, &cover, &writing_type, &author]
    )?.get(0);

    for tag_id in tags {
        transaction.execute(tag_insert_query, &[&tag_id, &new_writing_id])?;
    };

    transaction.commit()?;

    Ok(1)
}

pub fn get_one_writing(id: Uuid) -> Result<WritingFull, Error> {
    let mut client = crate::server::db::db_connect::get_database_client()?;
    
    let main_query = "
        SELECT w.id, w.title, w.content, w.cover, t.id, w.id_user
        FROM schema_seguridad.WRITING w
        INNER JOIN schema_seguridad.WRITING_TYPE t ON t.id = w.id_type
        WHERE w.id = $1;
    ";
    
    let main_row = client.query_one(main_query, &[&id])?; 
    
    let tags_query = "
        SELECT t.id, t.tag FROM schema_seguridad.WRITINGXTAG x
        INNER JOIN schema_seguridad.TAG t ON t.id = x.id_tag -- CORRECTED JOIN
        WHERE x.id_writing = $1;
    ";
    let tags_result = client.query(tags_query, &[&id])?;

    let memory_tags: Vec<Tag> = tags_result.iter().map(|tag| {
        Tag{ id: tag.get(0), name: tag.get(1) }
    }).collect();

    let comments_query = "
        SELECT c.id, a.name, c.content FROM schema_seguridad.COMMENT c
        INNER JOIN schema_seguridad.APP_USER a ON c.id_author = a.id 
        WHERE id_writing = $1;
    ";
    let comments_result = client.query(comments_query, &[&id])?;
    
    let memory_comments: Vec<Comment> = comments_result.iter().map(|comment| {
        Comment {
            id: comment.get(0),
            author: comment.get(1),
            content: comment.get(2)
        }
    }).collect();

    let writing = WritingFull{
        id: main_row.get(0),
        title: main_row.get(1),
        content: main_row.get(2),
        image: main_row.get(3),
        writing_type: main_row.get(4),
        author: main_row.get(5),
        tags: memory_tags,
        comments: memory_comments
    };
    
    Ok(writing)
}