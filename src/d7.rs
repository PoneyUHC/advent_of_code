
use crate::utils;
use std::{collections::HashMap, cell::RefCell, rc::Rc, fmt::Display};

#[derive(Debug)]
struct Wire(Rc<RefCell<Gate>>, Rc<RefCell<Gate>>, String);

impl Wire {
    fn get_value(&self, env: &mut Env) -> u16{
        let value = match self.2.parse::<u16>(){
            Ok(x) => x,
            Err(_) => {
                let saved = env.wires_values.get(&self.2);
                if saved.is_none() {
                    let computed = self.0.borrow().get_value(env);
                    env.wires_values.insert(self.2.clone(), computed);
                }
                let saved = env.wires_values.get(&self.2).unwrap();
                println!("{} value is {}", self.2, saved);
                *saved
            }
        };
        
        value
    }
}

#[derive(Debug)]
enum Gate {
    AssignWire(Rc<RefCell<Wire>>, Rc<RefCell<Wire>>),
    And(Rc<RefCell<Wire>>, Rc<RefCell<Wire>>, Rc<RefCell<Wire>>),
    Or(Rc<RefCell<Wire>>, Rc<RefCell<Wire>>, Rc<RefCell<Wire>>),
    LShift(Rc<RefCell<Wire>>, u16, Rc<RefCell<Wire>>),
    RShift(Rc<RefCell<Wire>>, u16, Rc<RefCell<Wire>>),
    Not(Rc<RefCell<Wire>>, Rc<RefCell<Wire>>),
    Loser
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String;
        match self {
            Gate::AssignWire(wire1, wire2) => {
                string = format!("{} -> {}", wire1.borrow().2, wire2.borrow().2)
            },
            Gate::And(wire1, wire2, wire3) => {
                string = format!("{} AND {} -> {}", wire1.borrow().2, wire2.borrow().2, wire3.borrow().2)
            },
            Gate::Or(wire1, wire2, wire3) => {
                string = format!("{} OR {} -> {}", wire1.borrow().2, wire2.borrow().2, wire3.borrow().2)
            },
            Gate::LShift(wire1, value, wire2) => {
                string = format!("{} LSHIFT {} -> {}", wire1.borrow().2, value, wire2.borrow().2)
            },
            Gate::RShift(wire1, value, wire2) => {
                string = format!("{} RSHIFT {} -> {}", wire1.borrow().2, value, wire2.borrow().2)
            },
            Gate::Not(wire1, wire2) => {
                string = format!("NOT {} -> {}", wire1.borrow().2, wire2.borrow().2)
            },
            _ => string = format!("Oups")
        }
        f.write_str(string.as_str())
    }
}

impl Gate {

    fn plug_wires(&mut self, self_ref: Rc<RefCell<Gate>>) {
        match self {
            Gate::AssignWire(wire1, wire2) => {
                wire1.borrow_mut().1 = self_ref.clone();
                wire2.borrow_mut().0 = self_ref.clone();
            },
            Gate::And(wire1, wire2, wire3) 
            | Gate::Or(wire1, wire2, wire3) => {
                wire1.borrow_mut().1 = self_ref.clone();
                wire2.borrow_mut().1 = self_ref.clone();
                wire3.borrow_mut().0 = self_ref.clone();
            },
            Gate::LShift(wire1, _, wire2)
            | Gate::RShift(wire1, _, wire2) => {
                wire1.borrow_mut().1 = self_ref.clone();
                wire2.borrow_mut().0 = self_ref.clone();
            },
            Gate::Not(wire1, wire2) => {
                wire1.borrow_mut().1 = self_ref.clone();
                wire2.borrow_mut().0 = self_ref.clone();
            },
            _ => ()
        }
    }
    
    
    fn get_value(&self, env: &mut Env) -> u16{
        println!("{}", self);
        match self {
            Gate::AssignWire(wire1, _) => {
                wire1.borrow().get_value(env)
            },
            Gate::And(wire1, wire2, _) => {
                wire1.borrow().get_value(env) & wire2.borrow().get_value(env)
            },
            Gate::Or(wire1, wire2, _) => {
                wire1.borrow().get_value(env) | wire2.borrow().get_value(env)
            },
            Gate::LShift(wire1, value, _) => {
                wire1.borrow().get_value(env) << value
            },
            Gate::RShift(wire1, value, _) => {
                wire1.borrow().get_value(env) >> value
            },
            Gate::Not(wire1, _) => {
                !wire1.borrow().get_value(env)
            },
            _ => 0
        }
    }
}

#[derive(Debug)]
struct Env {
    dummy_gate: Rc<RefCell<Gate>>,
    wires: HashMap<String, Rc<RefCell<Wire>>>,
    wires_values: HashMap<String, u16>
}

impl Env {
 
    fn get_wire(&mut self, name: String) -> Rc<RefCell<Wire>>{
        let wire = self.wires.get(&name);
        match wire {
            Some(w) => w.clone(),
            None => {
                self.wires.insert(
                    name.clone(), 
                    Rc::new(RefCell::new(Wire(self.dummy_gate.clone(), self.dummy_gate.clone(), name.clone())))
                );
                self.wires.get(&name).unwrap().clone()
            } 
        }
    }
}


fn parse_assign(args: Vec<&str>, env: &mut Env) -> Rc<RefCell<Gate>>{
    let gate = Gate::AssignWire(
        env.get_wire(String::from(args[0])), 
        env.get_wire(String::from(args[2]))
    );
    Rc::new(RefCell::new(gate))
}

fn parse_unary(args: Vec<&str>, env: &mut Env) -> Rc<RefCell<Gate>>{
    let gate = Gate::Not(
        env.get_wire(String::from(args[1])), 
        env.get_wire(String::from(args[3]))
    );
    Rc::new(RefCell::new(gate))
}

fn parse_binary(args: Vec<&str>, env: &mut Env) -> Rc<RefCell<Gate>>{
    let gate = match args[1] {
        "AND" => Gate::And(
            env.get_wire(String::from(args[0])), 
            env.get_wire(String::from(args[2])), 
            env.get_wire(String::from(args[4]))
        ),
        "OR" => Gate::Or(
            env.get_wire(String::from(args[0])), 
            env.get_wire(String::from(args[2])), 
            env.get_wire(String::from(args[4]))
        ),
        "LSHIFT" => Gate::LShift(
            env.get_wire(String::from(args[0])), 
            args[2].parse::<u16>().unwrap(), 
            env.get_wire(String::from(args[4]))
        ),
        "RSHIFT" => Gate::RShift(
            env.get_wire(String::from(args[0])), 
            args[2].parse::<u16>().unwrap(), 
            env.get_wire(String::from(args[4]))
        ),
        _ => Gate::Loser
    };
    Rc::new(RefCell::new(gate))
}

fn parse_line(line: &str, env: &mut Env) -> Rc<RefCell<Gate>>{
    let args: Vec<&str> = line.split(' ').collect();
    match args.len() {
        3 => parse_assign(args, env),
        4 => parse_unary(args, env),
        5 => parse_binary(args, env),
        _ => env.dummy_gate.clone()
    }
}


pub fn d7() -> u32 {
    
    let input = utils::get_input("d7_input.txt");

    let mut env = Env {
        dummy_gate: Rc::new(RefCell::new(Gate::Loser)),
        wires: HashMap::new(),
        wires_values: HashMap::new()
    };

    let mut gates = Vec::<Rc<RefCell<Gate>>>::new();

    for line in input.lines() {
        let gate = parse_line(line, &mut env);
        gate.borrow_mut().plug_wires(gate.clone());
        gates.push(gate);
    }
    
    env.get_wire(String::from("a")).borrow().get_value(&mut env) as u32
}
