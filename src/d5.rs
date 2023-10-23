
use crate::utils;

const FORBIDDEN : &'static[&str] = &["ab", "cd", "pq", "xy"];
const VOWELS : &'static[&str] = &["a", "e", "i", "o", "u"];

fn is_nice(string: &str) -> bool {
    
    for bad in FORBIDDEN.iter() {
        if string.contains(bad) {
            return false;
        }
    }
    
    let mut n_vowels = 0;
    let mut prev_letter = '/';
    let mut consecutive = false;
    for c in string.chars() {
        let mut str = [0u8; 4];
        if VOWELS.contains(&&*c.encode_utf8(&mut str)) {
            n_vowels += 1;
        }
        if c == prev_letter {
            consecutive = true;
        }
        prev_letter = c;
    }
    if n_vowels < 3 {
        return false;
    }
    
    consecutive
}


pub fn d5() -> u32 {
    
    let input = utils::get_input("d5_input.txt");

    let n_nice= input.lines().into_iter()
        .map(|line| is_nice(line))
        .filter(|nice| *nice)
        .count();
    
    return n_nice as u32;
}



fn is_nice_2(string: &str) -> bool {
    
    let mut found = false;
    for i in 0..(string.len()-3) {
        let substring = &string[i..i+2];
        if let Some(_) = string[i+2..].find(substring){
            found = true;
            break;
        }
    }
    
    if !found {
        return false;
    }
    
    found = false;
    let bytes = string.as_bytes();
    for i in 0..(string.len()-2){
        if bytes[i] == bytes[i+2] {
            found = true;
            break;
        }
    }
    
    found
}


pub fn d5_2() -> u32 {
    
    let input = utils::get_input("d5_input.txt");
    
    let n_nice= input.lines().into_iter()
        .map(|line| is_nice_2(line))
        .filter(|nice| *nice)
        .count();
    
    return n_nice as u32;
}