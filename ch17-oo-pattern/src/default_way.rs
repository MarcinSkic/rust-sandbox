pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_update_text() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn can_update_text(&self) -> bool {
        false
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn can_update_text(&self) -> bool {
        true
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approves: 0 })
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approves: i32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approves += 1;

        if self.approves != 2 {
            return self;
        } else {
            return Box::new(Published {});
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_is_available_only_in_published() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    #[test]
    fn text_can_be_changed_only_in_draft() {
        let mut post = Post::new();

        post.add_text("Changed in draft");

        post.request_review();

        post.add_text("Changed in review");

        post.approve();
        post.approve();

        post.add_text("Changed in published");

        assert_eq!("Changed in draft", post.content());
    }

    #[test]
    fn reject_works() {
        let mut post = Post::new();

        post.add_text("Changed in draft");

        post.request_review();

        post.reject();

        post.add_text("\nChanged after rejecting");

        post.request_review();

        post.approve();
        post.approve();

        assert_eq!("Changed in draft\nChanged after rejecting", post.content());
    }
}
