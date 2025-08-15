use redis::{Client, RedisResult};
use crate::{Fortune, FortuneStore};
use std::sync::OnceLock;

static REDIS_CLIENT: OnceLock<Option<Client>> = OnceLock::new();

pub async fn init() {
    let redis_dns = std::env::var("REDIS_DNS");
    if redis_dns.is_err() {
        println!("redis config not set");
        REDIS_CLIENT.set(None).unwrap();
        return;
    }

    let redis_url = format!("redis://{}:6379", crate::utils::get_env("REDIS_DNS", "localhost"));

    for attempt in 1..=5 {
        match Client::open(redis_url.as_str()) {
            Ok(client) => {
                match client.get_connection() {
                    Ok(_) => {
                        REDIS_CLIENT.set(Some(client)).unwrap();
                        println!("Successfully connected to Redis");
                        return;
                    }
                    Err(e) => {
                        eprintln!("Attempt {}: redis connection failed: {}", attempt, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Attempt {}: redis client creation failed: {}", attempt, e);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    eprintln!("Failed to connect to redis after 5 attempts");
    REDIS_CLIENT.set(None).unwrap();
}

pub async fn get_client() -> Option<Client> {
    REDIS_CLIENT.get().and_then(|opt| opt.as_ref().cloned())
}

pub async fn load_fortunes(client: &Client, store: FortuneStore) {
    let mut conn = match client.get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to get Redis connection: {}", e);
            return;
        }
    };

    let keys: RedisResult<Vec<String>> = redis::cmd("HKEYS").arg("fortunes").query(&mut conn);

    match keys {
        Ok(fortune_keys) => {
            println!("*** loading redis fortunes:");
            let mut store_write = store.write().await;

            for key in fortune_keys {
                let message: RedisResult<String> = redis::cmd("HGET")
                    .arg("fortunes")
                    .arg(&key)
                    .query(&mut conn);

                match message {
                    Ok(msg) => {
                        let fortune = Fortune {
                            id: key.clone(),
                            message: msg.clone(),
                        };
                        store_write.insert(key.clone(), fortune);
                        println!("{} => {}", key, msg);
                    }
                    Err(e) => {
                        eprintln!("redis hget failed: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("redis hkeys failed: {}", e);
        }
    }
}

pub async fn get_fortune(client: &Client, key: &str) -> RedisResult<String> {
    let mut conn = client.get_connection()?;
    redis::cmd("HGET")
        .arg("fortunes")
        .arg(key)
        .query(&mut conn)
}

pub async fn set_fortune(client: &Client, key: &str, message: &str) -> RedisResult<()> {
    let mut conn = client.get_connection()?;
    redis::cmd("HSET")
        .arg("fortunes")
        .arg(key)
        .arg(message)
        .query(&mut conn)
}
