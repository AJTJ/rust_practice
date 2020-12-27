// Add a reject method that changes the post’s state from PendingReview back to Draft.
// Require two calls to approve before the state can be changed to Published.
// Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approvals: u32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approvals: 0,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        let new_content = self
            .state
            .as_ref()
            .unwrap()
            .content_changed(&self.content, text);
        self.content = new_content.clone();
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approvals(&mut self) -> u32 {
        self.approvals
    }
    pub fn approve(&mut self) {
        self.approvals = self.approvals + 1;
        println!("post approved, number of approvals: {}", self.approvals());
        if self.approvals() >= 2 {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content_changed(&self, content: &str, _text: &str) -> String {
        println!("can't change content if not Draft");
        content.to_string()
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Draft {}
impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        "Is Draft"
    }

    fn content_changed(&self, _content: &str, text: &str) -> String {
        text.to_string()
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
