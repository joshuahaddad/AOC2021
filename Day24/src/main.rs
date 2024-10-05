use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn mul(state: &HashMap<String, i32>, to: String, arg: String) -> i32 {
    if state.contains_key(&arg) {
        return state[&to]*state[&arg];
    }
    else {
        return state[&to]*arg.parse::<i32>().unwrap();
    }
}

fn add(state: &HashMap<String, i32>, to: String, arg: String) -> i32 {
    if state.contains_key(&arg) {
        return state[&to]+state[&arg];
    }
    else {
        return state[&to]+arg.parse::<i32>().unwrap();
    }
}

fn div(state: &HashMap<String, i32>, to: String, arg: String) -> i32 {
    if arg.parse::<i32>().unwrap() == 0 {
        panic!();
    }
    if state.contains_key(&arg) {
        return state[&to]/state[&arg];
    }
    else {
        return state[&to]/arg.parse::<i32>().unwrap();
    }
}

fn modulo(state: &HashMap<String, i32>, to: String, arg: String) -> i32 {
    if arg.parse::<i32>().unwrap() <= 0 || state[&to] < 0 {
        panic!()
    }
    
    if state.contains_key(&arg) {
        return state[&to]%state[&arg];
    }
    else {
        return state[&to]%arg.parse::<i32>().unwrap();
    }
}

fn eql(state: &HashMap<String, i32>, to: String, arg: String) -> i32 {
    
    if state.contains_key(&arg) {
        return (state[&to]==state[&arg]) as i32;
    }
    else {
        return (state[&to]==arg.parse::<i32>().unwrap()) as i32;
    }
}

fn execute_instrs(instructions: &String, model: Vec<i32>){
    let mut state: HashMap<String, i32> = HashMap::new();
    state.insert("x".to_string(), 0);
    state.insert("y".to_string(), 0);
    state.insert("z".to_string(), 0);
    state.insert("w".to_string(), 0);
    let mut model_i = 0;

    for line in instructions.lines(){
        let instr = &line[0..3];
        let to = line[4..5].to_string();
        match instr {
            "inp" => {*state.get_mut(&to).unwrap() = model[model_i]; model_i += 1;},
            "mul" => {*state.get_mut(&to).unwrap() = mul(&state, to.clone(), arg)},
            "add" => {*state.get_mut(&to).unwrap() = add(&state, to.clone(), arg)},
            "div" => {*state.get_mut(&to).unwrap() = div(&state, to.clone(), arg)},
            "mod" => {*state.get_mut(&to).unwrap() = modulo(&state, to.clone(), arg)},
            "eql" => {*state.get_mut(&to).unwrap() = eql(&state, to.clone(), arg)},
            _ => panic!()
        };
        println!("{:?}", state);
    }
    println!("{:?}", state);
}


fn main() {
    let model = vec![1,3,5,7,9,2,4,6,8,9,9,9,9,9,];
    let instructions = fs::read_to_string("./src/test.txt").expect("Error reading file");
    execute_instrs(&instructions, model);
    //s.process_instruction("inp".to_string(), "x".to_string(), "y".to_string());
}

