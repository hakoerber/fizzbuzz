fn main() {
     (1..101).map(|i| match (i % 3, i % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        _ => i.to_string(),
    }).map(|i| println!("{0}", i)).for_each(drop);
}
