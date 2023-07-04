fn take_order() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    super::hosting::add_to_waitlist();
}

fn serve_order() {}

fn take_payment() {}
