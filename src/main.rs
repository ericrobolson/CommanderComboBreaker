use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

fn main() {
    let client = reqwest::blocking::Client::new();

    let mut combos = vec![];
    let mut card_counts = HashMap::new();

    // Commander spellbook
    let mut search = format!("https://commanderspellbook.com/search/?q=card%3A%22Ghave%2C+Guru+of+Spores%22+ci%3Awgb+legal%3Acommander&page=1");
    let mut keep_crawling = true;
    while keep_crawling {
        println!("{}", search);

        let response = client.get(&search).send().unwrap();
        let body = response.text().unwrap();

        // Parse the HTML from commander spellbook
        let selector = scraper::Selector::parse("div.py-1").unwrap();
        let document = scraper::Html::parse_document(&body);
        for element in document.select(&selector) {
            println!("COMBO");
            let name_selector = scraper::Selector::parse("div.card-name span").unwrap();
            let mut combo = vec![];
            for name_element in element.select(&name_selector) {
                let name = name_element.text().collect::<Vec<_>>().join("");
                println!("  {}", name);

                combo.push(name.clone());
                let count = card_counts.entry(name).or_insert(0);
                *count += 1;
            }

            if !combo.is_empty() {
                combos.push(combo);
            }
        }
        // TODO: fetch next page

        //
        let next_selector = scraper::Selector::parse("button.forward-button").unwrap();
        let next = document.select(&next_selector).next();
        keep_crawling = next.is_some();

        if keep_crawling {
            println!("{:?}", search);
            let mut split = search.split("&page=");
            let url = split.next();
            let page = split.next();

            println!("{:?} , {:?}", url, page);

            // Parse the page number as an integer
            let page = page.unwrap().parse::<u32>().unwrap();
            let next = page + 1;
            search = format!("{}&page={}", url.unwrap(), next);
        }
    }

    // Fetch from EDH Rec
    if (false == true) {
        let url = "https://edhrec.com/combos/ghave-guru-of-spores";
        let response = client.get(url).send().unwrap();
        let body = response.text().unwrap();
        let document = scraper::Html::parse_document(&body);

        let selector = scraper::Selector::parse("div.Grid_grid__EAPIs").unwrap();
        for element in document.select(&selector) {
            println!("COMBO EDHREC");
            let name_selector = scraper::Selector::parse("span.Card_name__Mpa7S").unwrap();
            let mut combo = vec![];
            for name_element in element.select(&name_selector) {
                let name = name_element.text().collect::<Vec<_>>().join("");
                println!("  {}", name);

                combo.push(name.clone());
                let count = card_counts.entry(name).or_insert(0);
                *count += 1;
            }

            if !combo.is_empty() && combo.len() < 3 {
                combos.push(combo);
            }
        }

        // Fetch next page?
    }

    combos.dedup();
    println!("{:#?}", combos);

    let mut cards = vec![];
    for (name, count) in card_counts {
        cards.push((name, count));
    }

    cards.sort_by(|a, b| b.1.cmp(&a.1));
    for (name, count) in cards.iter() {
        println!("{}: {}", name, count);
    }

    // Print out top 10 cards
    println!("Top 10 cards");
    for (name, count) in cards.iter().take(10) {
        println!("  {}: {}", name, count);
    }

    // First: https://edhrec.com/commanders/ghave-guru-of-spores
    // for combo in combos {
    //     println!("Combo: {:?}", combo);
    //
    // Second: https://edhrec.com/combos/ghave-guru-of-spores
}

fn build_search() {
    let commander_name = "Ghave, Guru of Spores";
    let color = "wgb";

    let search = format!("https://commanderspellbook.com/search/?q=card%3A%22Ghave%2C+Guru+of+Spores%22+ci%3Awgb+legal%3Acommander");

    let mut output = String::new();
    let example: &str = r#"https://commanderspellbook.com/search/?q=card%3A%22Ghave%2C+Guru+of+Spores%22+ci%3Awgb+legal%3Acommander"#;

    let decoded = url_escape::decode_to_string(example, &mut output);
    println!("{}", example);
    println!("{}", output);

    // let mut encoded_commander = String::new();
    // url_escape::encode_component_to_string(commander_name, &mut encoded_commander);
    // let encoded_commander = encoded_commander.replace("%20", "+");
    // let mut encoded_color = String::new();
    // url_escape::encode_component_to_string(color, &mut encoded_color);
    // let encoded_color = encoded_color.replace("%20", "+");

    // let mut query_string = String::new();
    // let query_string = format!("card:+ci:COLOR+legal:commander")
    // url_escape::encode_component_to_string(
    //     &"card:\"COMMANDER\"+ci:COLOR+legal:commander",
    //     &mut query_string,
    // );

    // let query_string = query_string
    //     .replace("COMMANDER", &encoded_commander)
    //     .replace("COLOR", &encoded_color);

    // let output = format!("https://commanderspellbook.com/search/?q={}", query_string);
    // println!("{}", output);
}

fn commander_spellbook_search(commander_name: &str, color: &str) -> String {
    let commander_name = html_escape::encode_text(commander_name);
    let color = html_escape::encode_text(color);
    format!(
        "https://commanderspellbook.com/search/?q=card%3A%22{}%22+ci%3A{}+legal%3Acommander",
        commander_name, color
    )
}
