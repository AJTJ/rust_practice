// A blog post starts as an empty draft.
// When the draft is done, a review of the post is requested.
// When the post is approved, it gets published.
// Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Today, I ate french toast");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Today, I ate french toast", post.content());
}
