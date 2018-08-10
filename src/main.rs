use std::io;

mod problem001;
mod problem002;
mod problem003;

fn main() {
    println!("Which Project Euler Problem to solve?");

    let mut project: u32;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        project = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Which Project Euler Problem to solve?");
                continue
            },
        };

        if project > 3 {
            println!("Which Project Euler Problem to solve?");
            continue
        }

        break;
    }


    println!("Solving problem: {}", project);
    
    if project == 1 {
        problem001::main();
    } else if project == 2 {
        problem002::main();
    } else if project == 3 {
        problem003::main();
    }
}