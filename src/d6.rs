
use std::cmp;

use crate::utils;

enum ECommand {
    TurnOn((u32, u32, u32, u32)),
    TurnOff((u32, u32, u32, u32)),
    Toggle((u32, u32, u32, u32)),
    FailMiserably
}


fn parse_coords(args: Vec<&str>) -> (u32, u32, u32, u32) {
    let min: Vec<u32> =  args.get(0).unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    let max: Vec<u32> = args.get(2).unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect();
    (min[0], min[1], max[0], max[1])
}


fn parse_toggle(args: Vec<&str>) -> ECommand {
    ECommand::Toggle(parse_coords(args))
}

fn parse_turn(args: Vec<&str>) -> ECommand {
    let on_off = args[0];
    return match on_off {
        "on" => ECommand::TurnOn(parse_coords(args[1..].to_vec())),
        "off" => ECommand::TurnOff(parse_coords(args[1..].to_vec())),
        _ => ECommand::FailMiserably
    };
}

fn parse_command(string: &str) -> ECommand {
    
    let args: Vec<&str> = string.split(' ').collect();
    let first = args[0];
    return match first {
        "turn" => parse_turn(args[1..].to_vec()),
        "toggle" => parse_toggle(args[1..].to_vec()),
        _ => ECommand::FailMiserably
    }
}


pub fn d6() -> u32 {
    
    let input = utils::get_input("d6_input.txt");

    let mut lights = [[false; 1000]; 1000];
    
    for line in input.lines() {
        let command = parse_command(line);
        match command {
            ECommand::TurnOn((x_min, y_min, x_max, y_max)) => {
                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        lights[i as usize][j as usize] = true;
                    }
                }
            },
            ECommand::TurnOff((x_min, y_min, x_max, y_max)) => {
                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        lights[i as usize][j as usize] = false;
                    }
                }
            },
            ECommand::Toggle((x_min, y_min, x_max, y_max)) => {
                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        lights[i as usize][j as usize] = !lights[i as usize][j as usize];
                    }
                }
            },
            ECommand::FailMiserably => ()
        }
    }
    
    lights.iter().map(|line| line.iter().filter(|v| **v).count() as u32).sum()
}


pub fn d6_2() -> u32 {
    
    let input = utils::get_input("d6_input.txt");

    let mut lights = [[0i32; 1000]; 1000];
    
    for line in input.lines() {
        let command = parse_command(line);
        match command {
            ECommand::TurnOn((x_min, y_min, x_max, y_max)) => {
                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        lights[i as usize][j as usize] += 1;
                    }
                }
            },
            ECommand::TurnOff((x_min, y_min, x_max, y_max)) => {
                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        lights[i as usize][j as usize] = cmp::max(lights[i as usize][j as usize] - 1, 0);
                    }
                }
            },
            ECommand::Toggle((x_min, y_min, x_max, y_max)) => {
                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        lights[i as usize][j as usize] += 2;
                    }
                }
            },
            ECommand::FailMiserably => ()
        }
    }
    
    lights.iter().map(|line| line.iter().sum::<i32>()).sum::<i32>() as u32
}