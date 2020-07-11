use traits::NewsArticle;
use traits::Summary;
use traits::Tweet;

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 tweet: {}", tweet.summarize());

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The P Pengs once again are the best \
    hockey team in the NHL.",
    ),
  };

  println!("article is: {}", article.summarize());
}
