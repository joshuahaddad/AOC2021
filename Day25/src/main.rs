use std::fs;
use std::collections::HashSet;

struct Environment {
    length: i32,
    width: i32,
    epochs: i32,
    vert_movers: HashSet<(i32, i32)>,
    horiz_movers: HashSet<(i32, i32)>
}

impl Environment {
    pub fn new(data: String) -> Environment {
        let mut y = 0;
        let x = data.lines().next().unwrap().len() as i32;
        let mut vert_movers: HashSet<(i32, i32)> = HashSet::new();
        let mut horiz_movers: HashSet<(i32, i32)> = HashSet::new();

        for (y_i, line) in data.lines().enumerate(){
            for (x_i, c) in line.chars().enumerate(){
                if c == '>' {
                    horiz_movers.insert((x_i as i32, y_i as i32));
                }
                else if c == 'v' {
                    vert_movers.insert((x_i as i32, y_i as i32));
                }
            }
            y += 1;
        }
        
        Environment {
            length: x,
            width: y,
            epochs: 0,
            vert_movers: vert_movers,
            horiz_movers: horiz_movers
        }
    }

    pub fn do_moves(&mut self, x_mod: i32, y_mod: i32) -> bool{
        let mut add_vals: Vec<(i32, i32, i32, i32)> = Vec::new();
        let mut dir = &mut self.horiz_movers;
        let mut other_dir = & mut self.vert_movers;
        let mut move_occurred = false;

        if x_mod != 1 {
            dir = &mut self.vert_movers;
            other_dir = &mut self.horiz_movers;
        }

        for mover in dir.iter() {
            let (new_x, new_y) = ((mover.0+x_mod) % self.length, (mover.1+y_mod) % self.width);
            if dir.contains(&(new_x, new_y)) || other_dir.contains(&(new_x, new_y)){
                continue;
            }
            else {
                add_vals.push((mover.0, mover.1, new_x, new_y));
                move_occurred = true;
                
            }
        }

        
        for mover in &add_vals{
            dir.remove(&(mover.0, mover.1));
            dir.insert((mover.2, mover.3));
        }

        return move_occurred;
    }

    pub fn iterate(&mut self) -> bool{
        let horiz_move = self.do_moves(1,0);
        let vert_move = self.do_moves(0,1);
        self.epochs += 1;

        return (horiz_move || vert_move);
    }
}

fn main() {
    let input_env: String = fs::read_to_string("./src/input.txt").expect("Error while reading file");
    let mut initial_env = Environment::new(input_env);
    loop {
        let continue_iterating = initial_env.iterate();
        if !continue_iterating{
            break;
        }
    }
    println!("{}", initial_env.epochs);
}

#[test]
fn prop1(){
    let input_env: String = fs::read_to_string("./src/test.txt").expect("Error while reading file");
    let mut initial_env = Environment::new(input_env);
    loop {
        let continue_iterating = initial_env.iterate();
        if !continue_iterating{
            break;
        }
    }
    assert_eq!(initial_env.epochs, 58);
}