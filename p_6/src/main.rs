fn 
main() 
{
    println!("{}", ssd(100));
}

fn
ssd
(
	lim: u64
)
-> u64
{
	let mut diff = 0;
	let gauss = lim * (lim+1) / 2;
	for i in 1..=lim
	{
		diff = diff + (i * gauss - i*i);
	}
	diff
}
