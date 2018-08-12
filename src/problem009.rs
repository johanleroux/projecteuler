pub fn main() {
    for c in 1..500 {
        for b in 1..c {
            for a in 1..b {
                if (a+b+c) == 1000 {
                    if ((a as u32).pow(2) + (b as u32).pow(2)) == (c as u32).pow(2) {
                        println!("{}*{}*{} = {}", a, b, c, (a*b*c));
                        break;
                    }
                }
            }
        }
    }    
}
