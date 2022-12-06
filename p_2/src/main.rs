fn 
get_fibonacci_numbers
(
	limit: u64
)
-> Vec<u64>
{
	let mut fibonaccis: Vec<u64> = vec!(1u64,1u64);
	let mut count = 2;
	while fibonaccis[count-1] < limit
	{
		fibonaccis.push(
			fibonaccis[count-1] + fibonaccis[count-2]
		);
		count += 1;
	}
	fibonaccis.pop();
	return fibonaccis;
}

fn main() 
{
	let sum: u64 = get_fibonacci_numbers(4000000)
		.iter()
		.filter(|&x| x % 2 == 0)
		.sum();

	println!("Sum of even fibonacci numbers smaller than 4000000: {}", sum);
}
