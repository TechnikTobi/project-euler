fn 
main() 
{
	let mut f_a = vec![1u8];
	let mut f_b = vec![1u8];
	let mut index = 2;

	while f_a.len() < 1000
	{
		let temp = next_fib(&f_a, &f_b);
		f_b = f_a;
		f_a = temp;
		// println!("{}", f_a.clone().into_iter().rev().map(|digit| digit.to_string()).collect::<Vec<String>>().join(""));
		index += 1;
	}

	println!("{}", index);
}

fn
next_fib
(
	fib_n_1: &Vec<u8>,
	fib_n_2: &Vec<u8>,
)
-> Vec<u8>
{
	let mut fib_n = Vec::new();

	let mut carry = 0;
	for index in 0..fib_n_1.len()
	{
		let sum = 
			  fib_n_1.iter().nth(index).unwrap_or(&0)
			+ fib_n_2.iter().nth(index).unwrap_or(&0)
			+ carry;
		let digit = sum % 10;
		fib_n.push(digit);
		carry = (sum - digit) / 10;
	}

	while carry > 0
	{
		let digit = carry % 10; 
		fib_n.push(digit);
		carry = (carry - digit) / 10;
	}

	return fib_n;
}
