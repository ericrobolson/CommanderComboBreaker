use crate::{web_page::WebPage, Color};
use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
};

pub type Card = String;
pub type NumResults = u32;

enum CrawlerMsg {
    FoundCombo,
    Finished { result: CrawlerResult },
}

enum CrawlerThreadMsg {
    Stop,
}

pub struct CrawlerTask {
    pub result: Option<CrawlerResult>,
    receiver: Receiver<CrawlerMsg>,
    thread_sender: Sender<CrawlerThreadMsg>,
    handle: Option<std::thread::JoinHandle<()>>,
    combos_found: usize,
}
impl CrawlerTask {
    pub fn new(colors: Vec<Color>, card_name: String) -> Self {
        // spawn in background thread
        let (sender, receiver) = mpsc::channel();
        let (thread_sender, thread_receiver) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            let result = crawl(colors, card_name, sender.clone(), thread_receiver);
            sender.send(CrawlerMsg::Finished { result }).unwrap();
        });

        Self {
            thread_sender,
            handle: Some(handle),
            receiver,
            result: None,
            combos_found: 0,
        }
    }

    pub fn stop(&self) {
        self.thread_sender.send(CrawlerThreadMsg::Stop).unwrap();
    }

    pub fn update(&mut self) {
        for msg in self.receiver.try_iter() {
            match msg {
                CrawlerMsg::FoundCombo => {
                    self.combos_found += 1;
                }
                CrawlerMsg::Finished { result } => {
                    self.result = Some(result);
                }
            }
        }
    }

    pub fn combos_found(&self) -> usize {
        self.combos_found
    }
}

impl Drop for CrawlerTask {
    fn drop(&mut self) {
        self.handle.take().unwrap().join().unwrap();
    }
}

pub struct CrawlerResult {
    pub colors: Vec<Color>,
    pub card: Card,
    pub cards: Vec<(Card, NumResults)>,
    pub combos: Vec<Vec<Card>>,
}

fn crawl(
    colors: Vec<Color>,
    card: String,
    sender: Sender<CrawlerMsg>,
    receiver: Receiver<CrawlerThreadMsg>,
) -> CrawlerResult {
    let mut combos = vec![];
    let mut card_counts = HashMap::new();

    // Commander spellbook

    let color_search = colors
        .iter()
        .map(|color| match color {
            Color::White => "w",
            Color::Blue => "u",
            Color::Black => "b",
            Color::Red => "r",
            Color::Green => "g",
            Color::Colorless => "c",
        })
        .collect::<Vec<_>>()
        .join("");

    let color_search = format!("ci%3A{}", color_search);
    let card_search = html_escape::encode_text(&card);
    todo!("{}, {}", color_search, card_search);
    let mut search = format!("https://commanderspellbook.com/search/?q=card%3A%22Ghave%2C+Guru+of+Spores%22+{color_search}+legal%3Acommander&page=1");
    let mut keep_crawling = true;
    while keep_crawling {
        let web_page = WebPage::fetch(&search);
        let document = web_page.document();

        // Parse the HTML from commander spellbook
        let selector = scraper::Selector::parse("div.py-1").unwrap();
        for element in document.select(&selector) {
            let name_selector = scraper::Selector::parse("div.card-name span").unwrap();
            let mut combo = vec![];
            for name_element in element.select(&name_selector) {
                let name = name_element.text().collect::<Vec<_>>().join("");

                combo.push(name.clone());
                let count = card_counts.entry(name).or_insert(0);
                *count += 1;
            }

            if !combo.is_empty() {
                sender.send(CrawlerMsg::FoundCombo).unwrap();
                combos.push(combo);
            }
        }
        // fetch next page
        let next_selector = scraper::Selector::parse("button.forward-button").unwrap();
        let next = document.select(&next_selector).next();
        keep_crawling = next.is_some();

        for msg in receiver.try_iter() {
            match msg {
                CrawlerThreadMsg::Stop => {
                    keep_crawling = false;
                }
            }
        }

        if keep_crawling {
            let mut split = search.split("&page=");
            let url = split.next();
            let page = split.next();

            // Parse the page number as an integer
            let page = page.unwrap().parse::<u32>().unwrap();
            let next = page + 1;
            search = format!("{}&page={}", url.unwrap(), next);
        }
    }

    // // Fetch from EDH Rec
    // if (false == true) {
    //     let url = "https://edhrec.com/combos/ghave-guru-of-spores";
    //     let response = client.get(url).send().unwrap();
    //     let body = response.text().unwrap();
    //     let document = scraper::Html::parse_document(&body);

    //     let selector = scraper::Selector::parse("div.Grid_grid__EAPIs").unwrap();
    //     for element in document.select(&selector) {
    //         println!("COMBO EDHREC");
    //         let name_selector = scraper::Selector::parse("span.Card_name__Mpa7S").unwrap();
    //         let mut combo = vec![];
    //         for name_element in element.select(&name_selector) {
    //             let name = name_element.text().collect::<Vec<_>>().join("");
    //             println!("  {}", name);

    //             combo.push(name.clone());
    //             let count = card_counts.entry(name).or_insert(0);
    //             *count += 1;
    //         }

    //         if !combo.is_empty() && combo.len() < 3 {
    //             combos.push(combo);
    //         }
    //     }

    //     // Fetch next page?
    // }

    combos.dedup();

    let mut cards = vec![];
    for (name, count) in card_counts {
        cards.push((name, count));
    }

    // Sort by count, then by name
    cards.sort_unstable_by_key(|a| (a.1, a.0.clone()));
    cards.reverse(); // ensure highest count is first

    CrawlerResult {
        cards,
        combos,
        colors,
        card,
    }
}
