//A trait declaration
pub trait Summary {
  // this first version does NOT include a default implementation
  // fn summarize(&self) -> String
  fn summarize(&self) -> String {
    // default behaviour included
    format!("Read more from {}", self.summarize_author())
  }

  fn summarize_author(&self) -> String;
}

pub trait Display {
  fn add_word(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// without the custom implementation, it will use the default implementation
impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("the author: {}", self.author)
  }
}

impl Display for NewsArticle {
  fn add_word(&self) -> String {
    format!("The Display addWord {}", self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  // fn summarize(&self) -> String {
  //   format!("{}: {}", self.username, self.content)
  // }
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// The IMPL TRAIT syntax
// instead of a concrete type in the item parameter, we are calling an implementation on a type. Thus 'notify' can be called on any type that implements that trait.
pub fn notify(item: &impl Summary) {
  println!("Breaking News! {}", item.summarize());
}

// this is the TRAIT BOUND syntax (the longer form of the above)
pub fn notify2<T: Summary>(item: &T) {
  println!("Generic Breaking News! {}", item.summarize())
}

pub fn summarize_two(item1: &impl Summary, item2: &impl Summary) {
  println!(
    "article summary: {} \
  tweet summary: {} ",
    item1.summarize(),
    item2.summarize()
  )
}

// if we wanted to FORCE BOTH PARAMS TO HAVE THE SAME TYPE you must use TRAIT BOUND
// pub fn summarize_two_definitely_same<T: Summary>(item1: &T, item2: &T) {}

pub fn notify_and_add_word(item: &(impl Summary + Display)) {
  println!(
    "Summary: {} \
  Display {}",
    item.summarize(),
    item.add_word()
  );
}

pub fn sum_and_display(item: &(impl Summary + Display)) {
  println!("sum and display: {}", item.summarize())
}
//instead of THIS
// fn many_bound_traits<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// for a case where there are many trait bounds, one can use WHERE
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
