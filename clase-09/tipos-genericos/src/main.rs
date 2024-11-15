fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

//genericos en structs
struct Point<T,U> {
    x: T,
    y: U,
}

// genericos en métodos
impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
}

//genericos en enums
enum Option<T> {
    Some(T),
    None,
}

