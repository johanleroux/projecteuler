use std::io;
extern crate num_integer;

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

        if project > 1 {
            println!("Which Project Euler Problem to solve?");
            continue
        }

        break;
    }


    println!("Solving problem: {}", project);
    
    if project == 1 {
        problem_one();
    }
}

fn problem_one() {
    let mut multiples: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    
    for num in 1..1000  {
        let (_div3, _rem3) = num_integer::div_rem(num, 3);
        let (_div5, _rem5) = num_integer::div_rem(num, 5);

        if _rem3 == 0 || _rem5 == 0 {
            multiples.push(num);
            sum += num;
        }
    }

    // for x in &multiples {
    //     println!("{}", x);
    // }

    println!("sum = {}", sum);
}
