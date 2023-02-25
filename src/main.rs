use onomicon::samples::simple::Simple;

fn main() {
    let pairs = [
        ("one", "a thing"),
        ("a very long key for", "?"),
        ("12345", "hey"),
    ];

    let simples = pairs
        .iter()
        .map(|(k, v)| Simple::new(k, v))
        .collect::<Vec<_>>();

    for simple in simples {
        println!("{:?}", simple);
    }
}
