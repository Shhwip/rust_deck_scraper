use std::{thread,io};
use scraper::Html;
use std::time::Duration;
use rand::prelude::*;

// selects from the commanders.html file to go through all the top commanders
fn parse_commanders(file: &str) {
    let document = scraper::Html::parse_document(file);

}

fn scrape_deck(title: &str) {
    // this builds the url in the form "https://tappedout.net/deck-link"
    let mut decklist_url = String::from("https://tappedout.net");
    decklist_url.push_str(title);
    // debug line
    println!("{}", decklist_url);
    let deck_response = reqwest::blocking::get(decklist_url).unwrap().text().unwrap();
    let deck_doc = scraper::Html::parse_document(&deck_response);
    // selects all cards including tokens
    // TODO remove tokens
    let card_selector = scraper::Selector::parse("span.card>a").unwrap();
    // prints each card
    let card_titles = deck_doc.select(&card_selector).map(|x|
        x.value().attr("data-name").unwrap_or(""));
    let mut i = 0;
    for card_title in card_titles {
        println!("{}, {}", {i += 1; i}, card_title);
    }
}

fn select_top_decks(commander: &str) -> Map<scraper::html::Select<'_, '_>{
    let commander = "gluntch-the-bestower";
    let mut search_link = String::from("https://tappedout.net/mtg-decks/search/?q=&format=edh&general=");
    search_link.push_str(commander);
    search_link.push_str("&price_min=&price_max=&o=-Views&submit=Filter+results");
    let response = reqwest::blocking::get(search_link).unwrap().text().unwrap();
    let document: Html = scraper::Html::parse_document(&response);
    // selects the links for the top decks
    let deck_title_selector = scraper::Selector::parse("h3.deck-wide-header>a").unwrap();
    let deck_titles = document.select(&deck_title_selector).map(|x|
    x.value().attr("href").unwrap_or("none"));
    return deck_titles;
}

fn scrape_top_decks(){}

fn main() {

    let commander = "gluntch-the-bestower";
    let mut rng = rand::thread_rng();
    let mut wait_time: Vec<u64> = (300..1500).collect();
    // This goes to the page that lists the top decks for the selected commander
    // TODO select from the commanders.html file to go through all of the top commanders
    let deck_titles = select_top_decks(commander);
    // scrapes all the decks
    for title in deck_titles {
        scrape_deck(title);
        // to not ddos the site
        // TODO vary the times randomly to get around any potential throttling
        wait_time.shuffle(&mut rng);
        thread::sleep(Duration::from_millis(*wait_time.get(0).unwrap()));

    }
}
