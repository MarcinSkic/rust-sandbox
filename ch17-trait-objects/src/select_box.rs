use super::Draw;

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing select box {}x{} with options: {:#?}",
            self.width, self.height, self.options
        );
    }
}
