fn 
main() 
{
	println!("{}", to_ascii(1));
	println!("{}", to_ascii(10));
	println!("{}", to_ascii(20));
	println!("{}", to_ascii(21));
	println!("{}", to_ascii(91));
	println!("{}", to_ascii(99));
	println!("{}", to_ascii(100));
	println!("{}", to_ascii(101));
	println!("{}", to_ascii(121));
	println!("{}", to_ascii(342));
	println!("{}", to_ascii(342).len());
	println!("{}", to_ascii(115));
	println!("{}", to_ascii(115).len());
}

fn
irregular_to_ascii
(
	number: u64
)
-> String
{
	return match number
	{
		1    => "one",
		2    => "two",
		3    => "three",
		4    => "four",
		5    => "five",
		6    => "six",
		7    => "seven",
		8    => "eight",
		9    => "nine",
		10   => "ten",
		11   => "eleven",
		12   => "twelve",
		13   => "thirteen",
		14   => "fourteen",
		15   => "fifteen",
		16   => "sixteen",
		17   => "seventeen",
		18   => "eighteen",
		19   => "nineteen",
		20   => "twenty",
		30   => "thirty",
		40   => "forty",
		50   => "fifty",
		60   => "sixty",
		70   => "seventy",
		80   => "eighty",
		90   => "ninety",
		100  => "hundred",
		1000 => "thousand",
		_ => todo!()
	}.to_string();
}

fn
to_ascii
(
	number: u64
)
-> String
{

	if number <= 20
	{
		return irregular_to_ascii(number);
	}

	if number < 100
	{
		if number % 10 == 0
		{
			return irregular_to_ascii(number);
		}

		let big_number = number - number % 10;

		let mut numberString = irregular_to_ascii(big_number);
		numberString.push_str(&irregular_to_ascii(number % 10));
		return numberString;
	}

	if number < 1000
	{
		let ten_number = number % 100;
		let hundred_number = number - ten_number;

		let mut numberString = irregular_to_ascii(hundred_number / 100);
		numberString.push_str(&irregular_to_ascii(100));
		if ten_number > 0
		{
			numberString.push_str("and");
			numberString.push_str(&to_ascii(ten_number));
		}
		return numberString;
	}

	if number < 1000000
	{

	}

	todo!();
}