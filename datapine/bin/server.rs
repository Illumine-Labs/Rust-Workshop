use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt,AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use datapine;


fn connect_addr() -> SocketAddr {
    dotenv().ok();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let host = dotenv::var("DP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = dotenv::var("DP_PORT").unwrap_or_else(|_| "9000".to_string());

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    
    println!("Datapine {} listening on {}", VERSION, addr);
    addr 
}

// Test: curl -v telnet://0.0.0.0:9000
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(connect_addr()).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mut store = datapine::Store::new();
        process(stream, &mut store).await;
    }
}

async fn process(mut stream: TcpStream, db: &mut datapine::Store) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    loop {
        let mut command = String::new();
        if let Err(_) = reader.read_line(&mut command).await {
            println!("Error reading from client");
            return;
        }
        command = command.trim().to_string();
    
        println!("GOT: {}", command);

        if let Ok(command) = datapine::Cmd::parse(&command) {
            match command {
                datapine::Cmd::Ping => {
                    datapine::ping(&mut writer).await;
                }
                datapine::Cmd::Insert(key, values) => {
                    datapine::insert(db, key, values, &mut writer).await;
                }
                datapine::Cmd::Remove(key) => {
                    datapine::remove(db, key, &mut writer).await;
                }
                datapine::Cmd::Get(key) => {
                    datapine::get(db, key, &mut writer).await;
                }
                datapine::Cmd::Knn(key, k) => {
                    datapine::knn(db, key, k, &mut writer).await;
                }
                datapine::Cmd::Cos(key, k) => {
                    datapine::cos(db, key, k, &mut writer).await;
                }
                datapine::Cmd::Dump(file_path) => {
                    datapine::dump(db,file_path, &mut writer).await;
                }
            }
        } else {
            if let Err(_) = writer.write_all(b"Error Command\n").await {
                println!("Error sending response to client");
                return;
            }
        }
        
    }
}

