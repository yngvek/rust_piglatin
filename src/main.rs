// Convert strings to pig latin. 
// The first consonant of each word is moved to the end of the word and “ay” is added, 
// so “first” becomes “irst-fay.” 
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
// Keep in mind the details about UTF-8 encoding!

fn main() {

//todo: lag input - readLine
//todo: handle uppercase/lowercase

let word = String::from("hello");

let a_letter = 'a';

println!("Letter is a vowel: {} ", is_vowel(a_letter));
}

fn is_vowel(a_letter: char) -> bool
{   
    let l = a_letter;
    //let a_letter_lowercase = a_letter.to_lowercase().to_string();
   
    let vowels = ['a','e','i','o','u','y','æ','ø','å'];
    if vowels.iter().any(|&x| x == a_letter)
        {true}
    else 
        {false}
}

fn pigify_vowel(string: &str) -> &str
{
    //add hay to the end (“apple” becomes “apple-hay”)
    //let vowel_addon = "hay";
    //let formatted_vowel = format!("{}-{}",vowel,vowel_addon);
   "pigified" 
}

fn pigify_consonant(string: &str) -> &str
{
    //The first consonant of each word is moved to the end of the word and “ay” is added, 
   // so “first” becomes “irst-fay.” 
    // let consonant_addon = "ay";
    // let formatted_consonant = format!("{}-{}",consonant,consonant_addon);
    "pigified"
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
