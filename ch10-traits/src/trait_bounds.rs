// almost equal but in fun2 both items have to be the same some type that implements Summary
pub fn fun1(item1: &impl Summary, item2: &impl Summary) {}
pub fn fun2<T: Summary>(item1: &T, item2: &T) {}

// specifing more than one bound
pub fn fun3(item: &(impl Summary + Display)) {}
pub fn fun4<T: Summary + Display>(item: &T) {}

// where clause, these are equal
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
