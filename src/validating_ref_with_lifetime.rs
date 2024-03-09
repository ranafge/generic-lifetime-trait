#[allow(dead_code, unused, uncommon_codepoints)]
//life time is another kind of generic
// lifetime ensure that the reference are valid as long as need them.
// Reference in rust has to be lifetime

// Preventing Dangling references with lifetime

// The main aim of lifetime is to prevent dangling references

// pub fn validting_life_time() {
//     let reference;
//     {
//         let x  =  5;
//         // x doesn't live longer enough
//         reference = &x;
//         // The reference is valid as long as in this scope but out this scope it not valid. So it ref to dangling reference
//         println!("{}", reference);
//     }
//     println!("The value of r is {}", reference);
//     // The rust compiler has a borrow checker
// }

// &i32; references
// &'a i32; // a reference with an explicit lifetime
// &'a mut i32; // a mutalbe reference with an explicit lifetime

pub fn invalidating_life_time2() {
    let reference;
    let x = 5;
    reference = &x;
    println!("{}", reference);
}

pub fn generic_lifetime_in_function() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest str is {}", result);
}

pub fn longest(str: &str, str2: &str) -> i32 {
    if str.len() == str2.len() {
        return str.len() as i32;
    } else if str.len() > str2.len() {
        return str.len() as i32;
    } else {
        str2.len() as i32
    }
}

pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn longest_string2<'a>(x: &'a str, y: &'a str) -> &'a str {
    let s = "The longest string is long";
    let str2 = "txy";
    let result = longest_string(s, str2);
    println!("The longest str is {}", result);
    result
}

// Lifietime annotation in struct definitions
struct ImportantExcept<'a> {
    part: &'a str,
}

pub fn use_important_except() {
    let novel = "Call me Ishmeal. Some year ago...".to_string();
    let first_sentence = novel.split(".").next().expect("not found '.' ");
    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("{}", i.part);
}

// Lifetime Elision

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// trait boud and lifetime and generics

use std::fmt::Debug;
pub fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Debug
    {
        println!("Announcement!: {:?}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }