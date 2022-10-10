use lazy_static::lazy_static;
use regex::Regex;

fn match_deck_link(text: &str) -> &str {
    lazy_static! {
        static ref RE:Regex = Regex::new(r"/mtg-decks/[a-z-]*").unwrap();
    }
    let mat = RE.find(text).unwrap();
    println!("{}, {}", mat.start(), mat.end());
    return "test"
}

fn main() {


    let response = reqwest::blocking::get(
        "https://tappedout.net/mtg-decks/search/?q=&format=edh&general=gluntch-the-bestower&price_min=&price_max=&o=-Views&submit=Filter+results"
    )
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let deck_title_selector = scraper::Selector::parse("h3.deck-wide-header>a").unwrap();
    let deck_titles = document.select(&deck_title_selector).map(|x|
    x.value().attr("href").unwrap_or("none"));
    deck_titles
        .zip(1..5)
        .for_each(|(item, number)| println!("{}. {}", number, item));
    // TODO pull out the /mtg-decks/$DECKTITLE for the next scrape
    // let input = document.select(&deck_title_selector).next().unwrap();
    // print!("{}", input.value().attr("href").unwrap_or("none"));
}
