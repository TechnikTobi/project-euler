use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn 
main() 
{
	let read_result = read_input();

	if read_result.is_err()
	{
		panic!();
	}

	let input = read_result.unwrap();
	let cleaned_input = input.replace("\"", "");
	let mut names = cleaned_input.split(",").collect::<Vec<&str>>();
	names.sort();

	let mut total_score = 0;
	for (i, name) in names.iter().enumerate()
	{
		total_score += (i as u64 +1) * compute_value(name);
	}

	println!("{}", total_score);
}

fn
compute_value
(
	name: &str
)
-> u64
{
	name.chars().map(|character| character as u64 - 64).sum::<u64>()
}

fn
read_input
()
-> Result<String, Box<dyn std::error::Error>>
{
	let file = File::open(
		std::path::Path::new("./0022_names.txt")
	)?;
	let lines = BufReader::new(file).lines();

	for result_line in lines
	{
		return Ok(result_line?);
	}

	return Ok("".to_string());
}