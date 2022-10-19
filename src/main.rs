use lazy_static::lazy_static;
use regex::Regex;
use std::{thread,time,io};

// selects from the commanders.html file to go through all the top commanders
fn parse_commanders(file: &str) {
    let document = scraper::Html::parse_document(file);

}
fn match_deck_link(text: &str) -> &str {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"/mtg-decks/[a-z-]*").unwrap();
    }
    let mat = RE.find(text).unwrap();
    println!("{}, {}", mat.start(), mat.end());
    return "test"
}

fn main() {

    // This goes to the page that lists the top decks for the selected commander
    // TODO select from the commanders.html file to go through all of the top commanders
    let response = reqwest::blocking::get(
        "https://tappedout.net/mtg-decks/search/?q=&format=edh&general=gluntch-the-bestower&price_min=&price_max=&o=-Views&submit=Filter+results"
    )
        .unwrap()
        .text()
        .unwrap();
    // scrapes the html file
    let document = scraper::Html::parse_document(&response);
    // selects the links for the top decks
    let deck_title_selector = scraper::Selector::parse("h3.deck-wide-header>a").unwrap();
    let deck_titles = document.select(&deck_title_selector).map(|x|
    x.value().attr("href").unwrap_or("none"));

    // scrapes all the decks
    for l in deck_titles {
        // this builds the url in the form "https://tappedout.net/deck-link"
        let mut decklist_url = String::from("https://tappedout.net");
        decklist_url.push_str(l);
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
        // to not ddos the site
        // TODO vary the times randomly to get around any potential throttling
        thread::sleep_ms(500);

    }
    // let decklist_page = reqwest::blocking::get("https://tappedout.net")
}
