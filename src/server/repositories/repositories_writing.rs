use postgres::Error;
use uuid::Uuid;

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