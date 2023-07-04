pub mod point {

    #[derive(Debug)]
    pub struct Point(pub i32, i32);
    impl Point {
        pub fn origin() -> Self {
            Point(0, 0)
        }
    }
}
fn main() {
    let mut p = point::Point::origin();
    p.0 += 1;
    // debug + println ignores accesiblity, p.1 is private but it compiles here without problem
    println!("{p:?}",);
}
