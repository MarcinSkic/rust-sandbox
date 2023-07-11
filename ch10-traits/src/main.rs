mod aggregator;

use aggregator::{NewsArticle, Summary};

fn main() {
    let tweet = aggregator::returns_summarizable();

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    aggregator::notify(&article);

    println!("Summarized author of 5: {}", 5.summarize_author());
}
