pub fn main() {
    let mut number: u32;
    let mut highest: u32 = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            number = x * y;
            if is_palindrome(number.to_string()) {
                if number > highest {
                    highest = number;
                }
            }
        }
    }

    println!("{}", highest);
}

fn is_palindrome(word: String) -> bool {
    let forwards = word.chars();
    let backwards = word.chars().rev();

    if forwards.eq(backwards) {
        return true;
    }

    return false;
}