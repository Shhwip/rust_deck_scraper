


#[cfg(test)]
mod tests{
    #[test]
fn test_select_top_decks() {
    let commander = "gluntch-the-bestower";
    let mut search_link = String::from("https://tappedout.net/mtg-decks/search/?q=&format=edh&general=");
    search_link.push_str(commander);
    search_link.push_str("&price_min=&price_max=&o=-Views&submit=Filter+results");
    let test = String::from("https://tappedout.net/mtg-decks/search/?q=&format=edh&general=gluntch-the-bestower&price_min=&price_max=&o=-Views&submit=Filter+results");
    assert_eq!(search_link, test);
}
}
