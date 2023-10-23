use crate::utils;


pub fn d2() -> u32 {
   
   let input = utils::get_input("d2_input.txt");
   
   let mut total = 0;
   
   for line in input.lines() {
        let parsed: Vec<u32> = line.split("x").map(|s| s.parse::<u32>().unwrap()).collect();
        let (l, w, h) = (parsed[0], parsed[1], parsed[2]);
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;
        let vec = vec![lw, wh, hl];
        let bigger = vec.iter().min().unwrap();
        total += 2*lw + 2*wh + 2*hl + bigger;
   }
   
   total
}


pub fn d2_2() -> u32 {
   
    let input = utils::get_input("d2_input.txt");
    
    let mut total = 0;
    
    for line in input.lines() {
        let mut parsed: Vec<u32> = line.split("x").map(|s| s.parse::<u32>().unwrap()).collect();
        parsed.sort(); 
        total += 2*parsed[0] + 2*parsed[1] + parsed[0]*parsed[1]*parsed[2];
    }
    
    total
 }
 
