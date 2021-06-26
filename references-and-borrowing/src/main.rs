fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let mut s2 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length os '{}' is {}.", s1, len);

    change(&mut s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
