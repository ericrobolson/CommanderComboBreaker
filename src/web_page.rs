use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref DBLOCK: Mutex<i32> = Mutex::new(0i32);
}

#[derive(Debug, Clone)]
pub struct WebPage {
    pub id: i32,
    pub url: String,
    pub html_body: String,
}
impl WebPage {
    pub fn fetch(url: &str) -> Self {
        let lock = match DBLOCK.lock() {
            Ok(lock) => lock,
            Err(poisoned) => {
                // Sleep then fetch
                std::thread::sleep(std::time::Duration::from_secs(1));
                return Self::fetch(url);
            }
        };
        // Pull from db
        let db = rusqlite::Connection::open("ccb.sqlite").unwrap();
        let mut stmt = db
            .prepare("SELECT id, url, html_body FROM html_page where url = (?1)")
            .unwrap();

        let rows = stmt
            .query_map([url], |row| {
                Ok(Self {
                    id: row.get(0).unwrap(),
                    url: row.get(1).unwrap(),
                    html_body: row.get(2).unwrap(),
                })
            })
            .unwrap();

        for row in rows {
            return row.unwrap();
        }

        let body;

        let client = reqwest::blocking::Client::new();
        let response = client.get(url).send().unwrap();
        body = response.text().unwrap();

        db.execute(
            "INSERT INTO html_page (url, html_body) VALUES (?1, ?2)",
            [url.to_string(), body],
        )
        .unwrap();

        return Self::fetch(url);
    }

    pub fn document(&self) -> scraper::Html {
        scraper::Html::parse_document(&self.html_body)
    }
}
