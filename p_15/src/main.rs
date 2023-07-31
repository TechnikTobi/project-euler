fn 
main() 
{
	println!("Hello, world! {}", count_moves(3,3));
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

	// return count_moves(avail_right, avail_down-1) + count_moves(avail_right-1, avail_down);
	return 
		2 * count_moves(avail_right-1, avail_down-1) + 
		2 * 
}
