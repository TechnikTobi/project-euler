fn 
main() 
{
	let n = 2000000;

	let mut gen = RandomGenerator::new();

	// Generate two starting points
	let a = Point::new(&mut gen);
	let b = Point::new(&mut gen);

	// Compute their distance as starting distance
	let mut dist = a.distance(&b);

	// Make vector that stores points generated so far
	let mut points = vec![a.clone(), b.clone()];

	// Generate new points...
	for i in 0..(n-2)
	{
		println!("{}", i);
		let new = Point::new(&mut gen);

		// ... and compute shortest distances
		for point in &points
		{

			// Replace if better
			if point.distance(&new) < dist
			{
				dist = point.distance(&new);
			}
		}

		// Add newly generated point for next round
		points.push(new);
	}

	println!("{}", dist.sqrt());
}

#[derive(Copy, Clone)]
struct
Point
{
	x: i64,
	y: i64
}

impl
Point
{
	fn
	new
	(
		gen: &mut RandomGenerator
	)
	-> Point
	{
		Point {x: gen.next() as i64, y: gen.next() as i64}
	}

	fn
	distance
	(
		&self,
		other: &Point
	)
	-> f64
	{
		(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64)
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

