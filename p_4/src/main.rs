fn
is_palindrome
(
	number: u64
)
-> bool
{
	let as_string = number.to_string();
	let string_len = as_string.len();

	for i in 0..string_len/2
	{
		if as_string.chars().nth(i) != as_string.chars().nth(string_len-i-1)
		{
			return false;
		}
	}

	return true;
}

fn main() 
{
	println!("{:?}", is_palindrome(123));
	println!("{:?}", is_palindrome(9009));
	println!("{:?}", is_palindrome(909));
	println!("{:?}", is_palindrome(908));
}
