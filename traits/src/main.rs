use traits::notify;
use traits::notify2;
use traits::notify_and_add_word;
use traits::sum_and_display;
use traits::summarize_two;
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

  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The P Pengs once again are the best \
    hockey team in the NHL.",
    ),
  };

  // using IMPL TRAIT syntax in the return position
  // returns a type that is only specified by the trait
  // THUS whatever the fn returns must implement the Summary trait
  fn returns_summarizable() -> impl Summary {
    Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
    }
  }

  // summarize has a default method that calls summarize_author()
  println!("just tweet summarize: {}", tweet.summarize());
  println!("just article summarize: {}", article.summarize());
  notify(&article);
  notify2(&article);
  notify_and_add_word(&article);
  summarize_two(&article, &tweet);
  sum_and_display(&article);
  println!(
    "Return summarizable: {}",
    returns_summarizable().summarize()
  )
}
