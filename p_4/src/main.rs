fn
is_palindrome
(
	number: u64
)
{
	let as_string = number.to_string();
	let string_len = as_string.len();
	let middle = (string_len / 2) as usize;
	println!("{}", as_string);
	println!("{}", middle);
}

fn main() 
{
	is_palindrome(123);
}
