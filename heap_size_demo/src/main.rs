use heap_size::HeapSize;

#[derive(HeapSize)]
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
}

fn main() {
    let demo_1 = Demo {
        a: Box::new(77),
        b: 11,
        c: "hello",
        d: "world".to_string(),
    };
    println!(
        "heap size = {} + {} + {} + {} = {}",
        demo_1.a.heap_size_of_children(),
        demo_1.b.heap_size_of_children(),
        demo_1.c.heap_size_of_children(),
        demo_1.d.heap_size_of_children(),
        demo_1.heap_size_of_children()
    );
}
