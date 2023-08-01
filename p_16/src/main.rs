fn 
main() 
{
	let mut pow = 1;
	let mut digits = vec!["2".to_string()];

	while pow < 1000
	{
		pow += 1;
		let current_len = digits.len();
		let mut carry = 0;

		for i in 0..current_len
		{
			let parsed_digit = digits[i].parse::<u8>().unwrap();
			let new_carry = if parsed_digit >= 5 { 1 } else { 0 };
			digits[i] = ((parsed_digit * 2 + carry) % 10).to_string();
			carry = new_carry;
		} 

		if carry > 0
		{
			digits.push(carry.to_string());
		}

		// println!("{} {:?}", pow, digits);
	}

	println!("{}", digits.iter().map(|digit| digit.parse::<u64>().unwrap()).sum::<u64>());
}
