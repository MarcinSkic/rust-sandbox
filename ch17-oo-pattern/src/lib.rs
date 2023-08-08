pub mod default_way;
pub mod rust_way;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_way_works() {
        let mut post = rust_way::Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
