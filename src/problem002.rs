pub fn main() {
    let mut fib_sum = 0;
    let mut even_sum = 0;
	let mut last = 0;
	let mut current = 1;

	while fib_sum < 4000000 {
		fib_sum = last + current;
		last = current;
		current = fib_sum;

        if current % 2 == 0 {
            even_sum += current;
        }
	}

    println!("Sum of even-valued terms: {}", even_sum);
}
