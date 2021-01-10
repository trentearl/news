extern crate reqwest;
extern crate redis;

pub fn get_and_cache_url(url: &str, ttl: &usize) -> String {
    match get(url) {
        Some(t) => t,
        None => {
            let text = reqwest::get(url)
                .unwrap()
                .text()
                .unwrap();

            match set(url, &text, ttl) {
                Ok(_) => text,
                _ => text
            }
        }
    }
}


fn set(key: &str, value: &String, ttl: &usize) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con: redis::Connection = client.get_connection().unwrap();
    let _ : () = redis::cmd("SETEX")
        .arg(key)
        .arg(ttl.to_string())
        .arg(value)
        .query(&mut con)?;


    Ok(())
}

fn get(key: &str) -> Option<String> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con: redis::Connection = client.get_connection().unwrap();
    let v: Result<Option<String>, _> = redis::cmd("GET")
        .arg(key)
        .query(&mut con);

    match v {
        Ok(v) => v,
        _ => None
    }
}

