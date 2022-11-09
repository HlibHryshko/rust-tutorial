fn main() {
    let s = String::from("Hi there guys!");
    println!("{}", first_word(&s))
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    } 
    &s[..]
}
