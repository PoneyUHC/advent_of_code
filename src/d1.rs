use crate::utils;


pub fn d1() -> u32 {
    let input = utils::get_input("d1_input.txt");
    
    let mut actual_level = 0;
    
    for (pos, character) in input.chars().enumerate() {
        if character == '('{
            actual_level += 1;
        } else {
            actual_level -= 1;
        }
        
        if actual_level < 0 {
            return (pos+1) as u32;
        }
    }
    
    return 0;
}
