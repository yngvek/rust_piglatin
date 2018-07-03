// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
// Support for UTF-8
use std::io;
fn main() {
    //todo: handle string starting with a number
    //todo: what about 1 letter only, how to behave
    println!("This program piglatinifies a string. Please enter a string");
    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    println!(".....piglatinifying in progress..");

    let split_string: Vec<&str> = input_string.trim().split(' ').collect();

    for a_string in &split_string {
        if is_vowel(&a_string) {
            print!("{} ", pigify_vowel(&a_string));
        } else {
            print!("{} ", pigify_consonant(&a_string));
        }
    }
}

fn is_vowel(string: &str) -> bool {

        let letter_lowercase = string
            .chars()
            .nth(0)
            .expect("")
            .to_lowercase()
            .to_string();

    let vowels = ["a", "e", "i", "o", "u", "y", "æ", "ø", "å"];
    vowels.iter().any(|&x| x == letter_lowercase) 
}

fn pigify_vowel(string: &str) -> String {
    //add hay to the end (“apple” becomes “apple-hay”)
    let vowel_addon = "hay";
    format!("{}-{}", &string, vowel_addon)
}

fn pigify_consonant(string: &str) -> String {
    //The first consonant of each word is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.”
    let consonant_addon = "ay";

    let first_letter = &string.chars().nth(0).expect("");
    let rest_of_word: String = string.chars().skip(1).collect();

    format!("{}-{}{}", rest_of_word, first_letter, consonant_addon)
}

// fn main() {
// let t = 't';  //char
// let h = "hello world"; //string literal - &str
// let e = String::new(); //empty string

// let mut s = String::from("hello world");

// let data = "initial content";
// let s = data.to_string();  //string from string literal

// let x = String::from("initial contents"); //string from literal, one line.

// println!("{}", s)

// //add
// let mut s1 = String::from("foo");
// s1.push_str("bar");  //adds a string slice to string
// println!("{}", s1);

// let mut s2 = String::from("lo");
// s2.push('l'); //adds a single character
// println!("{}", s2);

// //concatenate with +
// let s3 = String::from("Hello, ");
// let s4 = String::from("world!");
// let s5 = s3 + &s4;  //deref coercion s4 - String - into a &str. s4 is turned into &s4[..]
// println!("{}", s5);
// println!("{}", s4);
// //println!("{}", s3); //parameter s3 has moved, and adding this line will not compile

// let s6 = String::from("tic");
// let s7 = String::from("tac");
// let s8 = String::from("toe");

// let s678 = s6 + "-" + &s7 + "-" + &s8;
// println!("{}", s678);

// //concatenate with format! Doetn's take ownership, like + does
// let s10 = String::from("tic");
// let s11= String::from("tac");
// let s12 = String::from("toe");
// let s101112 = format!("{}-{}-{}", s10, s11, s12);
// println!("{}", s101112);

// //indexing
// let len1 = String::from("Hola").len();
// println!("{}", len1);

// let len2 = String::from("Здравствуйте").len();
// println!("{}", len2);

// //slicing strings
// let hello = "Здравствуйте";
// let s13 = &hello[0..4]; //crashes
// //let s = &hello[0..1]; //crashes
// println!("{}", s13 );

// //iterate over strings
// for c in hello.chars() {
// println!("{}", h);
// }

// let hello2 = "Здравствуйте";
// for b in hello2.bytes()  {
//     println!("{}", b);
// }

//}
