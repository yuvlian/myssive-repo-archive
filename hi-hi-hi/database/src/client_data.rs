use crate::Arclite;
use sqlx::{query, query_as};

pub async fn insert_client_data(pool: &Arclite, client_data: (u32, i32, Vec<u8>)) {
    query("INSERT INTO client_data (id, r#type, data) VALUES (?, ?, ?)")
        .bind(client_data.0)
        .bind(client_data.1)
        .bind(client_data.2)
        .execute(pool.as_ref())
        .await
        .expect("failed inserting client data");
}

pub async fn get_all_client_data(pool: &Arclite) -> Vec<(i64, u32, i32, Box<[u8]>)> {
    let client_data = query_as::<_, (i64, u32, i32, Box<[u8]>)>(
        "SELECT client_data_id, id, r#type, data FROM client_data",
    )
    .fetch_all(pool.as_ref())
    .await
    .expect("failed fetch all client data");

    client_data
}
