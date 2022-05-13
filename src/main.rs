use std::io;

fn reverse_string(s: &str) -> String {
  s.chars().rev().collect()
}
fn main() {
    let mut input = String::new();
    println!("Enter a word: ");
    io::stdin().read_line(&mut input).expect("Not a valid str");
    let str1 = input.trim();
    let str2 = reverse_string(str1);
    println!("The reverse of the word {} is {}.", str1, str2);
    if str1 == str2{
        println!("it works, it is the same!");
    }else if str1 != str2{
        println!("it is not the same");
    }
    
}


