fn 
main() 
{
	let n = 10u64.pow(16);

	let mut counter = 61;
	let mut prime_checker = PrimeChecker::new();
	let mut current_positive_integer = vec![7, 5, 1u8];

	println!("{}", current_positive_integer.clone().into_iter().rev().map(|digit| digit.to_string()).collect::<Vec<_>>().join(""));

	incremet_int(&mut current_positive_integer);

	loop
	{
		let digit_sum: u64 = current_positive_integer.iter().map(|digit| *digit as u64).sum();
		if prime_checker.check(digit_sum)
		{
			counter += 1;
		}
	}

	println!("{}", current_positive_integer.clone().into_iter().rev().map(|digit| digit.to_string()).collect::<Vec<_>>().join(""));
}

fn
incremet_int
(
	vec: &mut Vec<u8>
)
{
	let mut carry = 2;
	for i in 0..vec.len()
	{
		let sum = vec.iter().nth(i).unwrap_or(&0) + carry;
		let digit = sum % 10;
		carry = (sum - digit) / 10;
		vec[i] = digit;
	}

	while carry > 0
	{
		let digit = carry % 10;
		carry = (carry - digit) / 10;
		vec.push(digit);
	}
}

fn
digit_sum
(

)
{

}

struct
PrimeChecker
{
	primes: Vec<u64>
}

impl
PrimeChecker
{
	fn
	new
	()
	-> PrimeChecker
	{
		PrimeChecker { primes: vec![2, 3, 5, 7] }
	}

	fn
	check
	(
		&mut self,
		candidate: u64
	)
	-> bool
	{
		while self.primes.last().unwrap() < (candidate as f64).sqrt()
		{

		}

		todo!()
	}
}