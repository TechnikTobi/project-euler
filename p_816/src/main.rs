fn 
main() 
{
	println!("Hello, world!");
	let mut gen = RandomGenerator::new();

	for i in 0..10
	{
		println!("{}", gen.next());
	}
}

struct
RandomGenerator
{
	state: u64
}

impl
RandomGenerator
{
	fn
	new
	()
	-> RandomGenerator
	{
		RandomGenerator
		{
			state: 290797
		}
	}

	fn
	next
	(
		&mut self
	)
	-> u64
	{
		let result = self.state;
		self.state = (self.state * self.state) % 50515093;
		return result;
	}
}

#[no_mangle]
pub extern "C" fn rust_function(arg1: i32, arg2: i32) -> i32
{
	arg1 + arg2
}
