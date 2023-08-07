use ch17_trait_objects::*;
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 32,
                height: 24,
                label: String::from("Guziczek"),
            }),
            Box::new(select_box::SelectBox {
                width: 100,
                height: 50,
                options: vec![
                    String::from("small"),
                    "medium".to_string(),
                    "large".to_string(),
                ],
            }),
        ],
    };
    screen.run();
}
