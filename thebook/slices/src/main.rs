fn main() {
    let _s = String::from("hello world");
    let s2 = "hello world";
    let word = first_word(s2);

    println!("{}", word);

    // can pass slice of string as a ref
    // point at same string as whole on heap, but points to certain index
    //let hello = &s[0..5];
    //let world = &s[6..11];
   
    //s.clear();
}

fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
} 
