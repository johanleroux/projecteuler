extern crate num_integer;

pub fn main() {
    let mut fib_sum = 0;
    let mut even_sum = 0;
	let mut last = 0;
	let mut current = 1;

	while fib_sum < 4000000 {
		fib_sum = last + current;
		last = current;
		current = fib_sum;

        let (_div, _rem) = num_integer::div_rem(current, 2);
        if _rem == 0 {
            even_sum += current;
        }
	}

    println!("Sum of even-valued terms: {}", even_sum);
}
