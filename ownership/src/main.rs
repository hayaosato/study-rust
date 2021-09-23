fn main() {
    let s = String::from("hello world");
    println!("{}", s);
    let hoge = hoge(&s[..]);
    println!("{}", hoge);
}

fn hoge(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
