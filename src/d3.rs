use std::collections::HashSet;


use crate::utils;


pub fn d3() -> u32 {
    
    let input = utils::get_input("d3_input.txt");
    
    let mut actual = (0,0);
    let mut visited = HashSet::new();

    visited.insert(actual);
       
    for code in input.chars() {
        actual = match code {
            '<' => (actual.0 - 1, actual.1),
            '>' => (actual.0 + 1, actual.1),
            '^' => (actual.0, actual.1 + 1),
            'v' => (actual.0, actual.1 - 1),
            _ => actual
        };
        visited.insert(actual);
    }
    
    visited.len() as u32
}




pub fn d3_2() -> u32 {
    
    let input = utils::get_input("d3_input.txt");
    
    let mut actual = [(0, 0), (0, 0)];
    let mut visited = HashSet::new();
    
    let mut who = 0;
    visited.insert(actual[who]);
       
    for code in input.chars() {
        actual[who] = match code {
            '<' => (actual[who].0 - 1, actual[who].1),
            '>' => (actual[who].0 + 1, actual[who].1),
            '^' => (actual[who].0, actual[who].1 + 1),
            'v' => (actual[who].0, actual[who].1 - 1),
            _ => actual[who]
        };
        visited.insert(actual[who]);
        who = 1 - who;
        
        println!("({},{})", actual[who].0, actual[who].1);
        
    }
    
    visited.len() as u32
}