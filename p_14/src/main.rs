fn 
main() 
{
	let mut longest_chain_producer = 1;
	let mut longest_chain_len = 1;

	for i in 1..10u64.pow(6)
	{
		let chain_length = get_length_of_chain(i);
		if chain_length > longest_chain_len
		{
			longest_chain_len = chain_length;
			longest_chain_producer = i;
			println!("{}", i);
		}
	}
	println!("Result: {}", longest_chain_producer);
}

fn
get_length_of_chain
(
	number: u64
)
-> usize
{
	if number == 1
	{
		return 1;
	}

	return 
		if number % 2 == 0
		{
			get_length_of_chain(number / 2)
		}
		else
		{
			get_length_of_chain(3*number + 1)
		}
		+1;
}
