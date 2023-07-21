use super::*;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::WriteHalf;

pub async fn ping(writer: &mut WriteHalf<'_>) {
    if let Err(_) = writer.write_all(b"pong\n").await {
        println!("Error sending response to client");
    }
}


pub async fn insert(
    db: &mut Store,
    key: String,
    values: Vec<f32>,
    writer: &mut WriteHalf<'_>,
) {
    db.insert(key, values);
    if let Err(_) = writer.write_all(b"OK\n").await {
        println!("Error sending response to client");
    }
}

pub async fn remove(
    db: &mut Store,
    key: String,
    writer: &mut WriteHalf<'_>,
) {
    db.remove(&key);
    if let Err(_) = writer.write_all(b"OK\n").await {
        println!("Error sending response to client");
    }
}

pub async fn get(
    db: &mut Store,
    key: String,
    writer: &mut WriteHalf<'_>,
) {
    if let Some(values) = db.get(&key) {
        let response = format!("{:?}\n", values);
        if let Err(_) = writer.write_all(response.as_bytes()).await {
            println!("Error sending response to client");
        }
    } else {
        if let Err(_) = writer.write_all(b"null\n").await {
            println!("Error sending response to client");
        }
    }
} 

pub async fn knn(
    db: &mut Store,
    key: String,
    k: usize,
    writer: &mut WriteHalf<'_>,
) {
    match db.get(&key) {
        Some(query_vector) => {
            let neighbors = db.knn(query_vector, k);
            let response = neighbors
                .into_iter()
                .map(|(id, vector)| format!("ID: {}, Vector: {:?}", id, vector))
                .collect::<Vec<String>>()
                .join("\n");
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
        None => {
            let response = "Key not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn cos(
    db: &mut Store,
    key1: String,
    key2: String,
    writer: &mut WriteHalf<'_>,
) {
    match (db.get(&key1.clone()), db.get(&key2.clone())) {
        (Some(vector1), Some(vector2)) => match db.cosine_similarity(&vector1, &vector2) {
            Some(similarity) => {
                let response = format!("Cosine Similarity: {:.4}\n", similarity);
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
            None => {
                let response = "Vectors are not compatible for cosine similarity\n";
                if let Err(_) = writer.write_all(response.as_bytes()).await {
                    println!("Error sending response to client");
                }
            }
        },
        _ => {
            let response = "One or both keys not found\n";
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}

pub async fn dump(db: &mut Store, file_path: String, writer: &mut WriteHalf<'_>) {
    match db.dump_to_file(&file_path) {
        Ok(_) => {
            let response = format!("Database dump successful: {}\n", file_path);
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
        Err(err) => {
            let response = format!("Error creating database dump: {}\n", err);
            if let Err(_) = writer.write_all(response.as_bytes()).await {
                println!("Error sending response to client");
            }
        }
    }
}