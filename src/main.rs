mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;

fn main() {
    let mut project: u32;

    loop {
        println!("Which Project Euler Problem to solve?");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        project = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("An error occured.");
                continue;
            },
        };

        println!("Solving problem: {}", project);
        
        if project == 1 {
            problem001::main();
        } else if project == 2 {
            problem002::main();
        } else if project == 3 {
            problem003::main();
        } else if project == 4 {
            problem004::main();
        } else if project == 5 {
            problem005::main();
        } else if project == 6 {
            problem006::main();
        } else if project == 7 {
            problem007::main();
        } else if project == 8 {
            problem008::main();
        } else if project == 9 {
            problem009::main();
        } else if project == 10 {
            problem010::main();
        } else {
            println!("Problem not available");
            continue;
        }

        break;
    }
}