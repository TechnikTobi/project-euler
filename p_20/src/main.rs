fn 
main() 
{
	let mut digits = vec!["1".to_string()];
	let factorial = 100;

	for factor in 1..=factorial
	{
		let mut carry = 0;
		let current_len = digits.len();
		for i in 0..current_len
		{
			let parsed_digit = digits[i].parse::<u64>().unwrap();
			let product = parsed_digit * factor + carry;
			digits[i] = (product % 10).to_string();
			carry = (product - product % 10) / 10;
		}

		while carry > 0 
		{
			digits.push((carry % 10).to_string());
			carry = (carry - carry % 10) / 10;
		}
	}

	println!("{}", digits.iter().map(|digit| digit.parse::<u64>().unwrap()).sum::<u64>());
}
