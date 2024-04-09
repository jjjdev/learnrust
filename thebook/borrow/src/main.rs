fn main() {
   let s1 = String::from("Hello");
   let len = calculate_length(&s1);

   println!("The length of '{}' is {}.", s1, len); 

   let mut s2 = String::from("Hello");
   change_string(&mut s2);
   println!("new string is {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}