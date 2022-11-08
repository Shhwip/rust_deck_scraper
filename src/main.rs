use std::thread;
use std::iter::Map;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scraper::Html;
use std::time::Duration;
use rand::prelude::*;

fn scrape_deck(title: &str) {
    // this builds the url in the form "https://tappedout.net/deck-link"
    let mut decklist_url = String::from("https://tappedout.net");
    decklist_url.push_str(title);
    // debug line
    println!("{}", decklist_url);
    let deck_response = reqwest::blocking::get(decklist_url).unwrap().text().unwrap();
    let deck_doc = scraper::Html::parse_document(&deck_response);
    // selects all cards including tokens
    // TODO: remove tokens
    // this is giving a bunch of repeats
    // the decks with primers, even hidden ones! make hundreds of dupes
    // the following deck:https://tappedout.net/mtg-decks/dominus-dreamcrusher-edition/
    // has 120 cards in the deck but the scraper picks up 582 cards because of the hidden primer
    let card_selector = scraper::Selector::parse("span.card>a").unwrap();
    // prints each card
    // TODO: place the cards into a database
    let card_titles = deck_doc.select(&card_selector).map(|x|
        x.value().attr("data-name").unwrap_or(""));
    let mut i = 0;
    for card_title in card_titles {
        println!("{}, {}", {i += 1; i}, card_title);
    }
}

fn select_top_decks(commander: &str) {
    // set up rng for the wait time between requests
    let mut rng = rand::thread_rng();
    let mut wait_time: Vec<u64> = (300..1500).collect();
    // assemble the link to the top decks of the chosen commander
    let mut search_link = String::from("https://tappedout.net/mtg-decks/search/?q=&format=edh&general=");
    search_link.push_str(commander);
    search_link.push_str("&price_min=&price_max=&o=-Views&submit=Filter+results");
    // debug line
    println!("{}", search_link);
    // turn the page into html
    let response = reqwest::blocking::get(search_link).unwrap().text().unwrap();
    let document: Html = scraper::Html::parse_document(&response);
    // selects the links for the top decks from the html page
    let deck_title_selector = scraper::Selector::parse("h3.deck-wide-header>a").unwrap();
    let deck_titles = document.select(&deck_title_selector).map(|x|
    x.value().attr("href").unwrap_or("none"));
        
    // scrapes all the decks
    for title in deck_titles {
        scrape_deck(title);
        // to not ddos the site
        // TODO vary the times randomly to get around any potential throttling
        wait_time.shuffle(&mut rng);
        thread::sleep(Duration::from_millis(*wait_time.get(0).unwrap()));
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn first_colon(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b':' {
            return i;
        }
    }

    s.len()
}

fn main() {

    // let mut commander = "gluntch-the-bestower";
    if let Ok(lines) = read_lines("./data/commanders.txt") {
        for line in lines {
            if let Ok(commander) = line {
                // this should be reading line by line but its picking up the wrong commanders/decks
                // this should be the first deck selected
                // https://tappedout.net/mtg-decks/atraxa-voice-of-infect/
                // this is the first deck selected
                // https://tappedout.net/mtg-decks/mikaeus-extreme-sub-20-budget-edh/
                let sanitized = &commander[..first_colon(&commander)];
                println!("{}",sanitized);
                select_top_decks(sanitized);
            }
        }
    }
}
