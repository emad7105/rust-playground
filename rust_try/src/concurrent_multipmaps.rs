

pub fn run() {
    let (book_reviews_r, mut book_reviews_w) = evmap::new();

    // review some books.
    book_reviews_w.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews_w.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews_w.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews_w.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");

// at this point, reads from book_reviews_r will not see any of the reviews!
    assert_eq!(book_reviews_r.len(), 0);
    // we need to refresh first to make the writes visible
    book_reviews_w.refresh();
    assert_eq!(book_reviews_r.len(), 4);
// reads will now return Some() because the map has been initialized
    assert_eq!(book_reviews_r.get("Grimms' Fairy Tales").map(|rs| rs.len()), Some(1));
}