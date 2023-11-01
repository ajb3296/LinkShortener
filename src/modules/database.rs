use mysql::{params, prelude::*, Pool, Error};


pub fn enroll_to_db(pool: &Pool, url: String) -> Result<u128, ()> {
    let mut conn = pool.get_conn().unwrap();

    let last_insert_id: u64 = conn.exec_drop(
        "
        INSERT INTO `urls` (url)
        VALUES (:url)
        ",
        params! {
            "url" => url
        },
    ).map(|_| conn.last_insert_id()).unwrap();

    if last_insert_id > 0 {
        Ok(last_insert_id as u128)
    } else {
        Err(())
    }
}

pub fn get_url_from_id(pool: &Pool, id: u128) -> Result<String, Error> {
    let mut conn = pool.get_conn().unwrap();

    let query = format!(
        "SELECT url FROM urls WHERE id = {} LIMIT 1",
        id
    );

    let result: Option<String> = conn.query_first(query).unwrap();

    match result {
        Some(url) => return Ok(url),
        None => return Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "No URL found"))),
    }
}