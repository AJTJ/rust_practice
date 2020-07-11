//A trait declaration
pub trait Summary {
  // this first version does NOT include a default implementation
  // fn summarize(&self) -> String
  fn summarize(&self) -> String {
    format!("Read more from {}", self.summarize_author())
  }

  fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// without the custom implementation, it will use the default implementation
impl Summary for NewsArticle {
  // fn summarize(&self) -> String {
  //   format!("{}, by {} ({})", self.headline, self.author, self.location)
  // }
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}
