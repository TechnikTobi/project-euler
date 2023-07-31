fn 
main() 
{
	println!("2:  {}", count_moves(2 ,2 ));
	println!("3:  {}", count_moves(3 ,3 ));
	println!("4:  {}", count_moves(4 ,4 ));
	println!("20: {}", count_moves(10,10));
}

fn
count_moves
(
	avail_right: usize,
	avail_down:  usize,
)
-> u64
{
	if avail_down == 0 || avail_right == 0
	{
		return 1;
	}

	let mut moves = 0;

	if avail_right == avail_down
	{
		for i in 1..=avail_right
		{
			moves += 2*count_moves(avail_right-i, avail_down-1);
		}
	}
	else
	{
		for i in 1..=avail_right
		{
			moves += count_moves(avail_right-i, avail_down-1);
		}

		for i in 1..=avail_down
		{
			moves += count_moves(avail_right-1, avail_down-i);
		}

	}

	return moves;
}
