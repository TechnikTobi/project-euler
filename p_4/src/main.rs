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
	let mut maximum = 0;

	for a in 100..=999
	{
		for b in a..=999
		{
			if is_palindrome(a*b)
			{
				if maximum < a*b
				{
					maximum = a*b;
				}
			}
		}
	}

	println!("{}", maximum);
}
