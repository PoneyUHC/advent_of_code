
use crate::utils;
use md5;

pub fn d4() -> u32 {
    
    let input = utils::get_input("d4_input.txt");
    let mut first = 0;
    
    for i in 0..1_000_000_000 {
        let to_hash = format!("{}{}", input.as_str(), i.to_string().as_str());
        let hash = md5::compute(to_hash);
        let string = format!("{:x}", hash);
        if string.starts_with("000000") {
            first = i;
            break;
        }
    }
    
    first    
}
