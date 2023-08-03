fn 
main() 
{
	let mut year = 1900;
	let mut month = 1;
	let mut day = 1;
	let mut weekday = 1;

	let mut sundays_on_first_of_month = 0;

	while is_twentieth_century(year, month, day)
	{
		if day == 1 && weekday == 0
		{
			sundays_on_first_of_month += 1;
			println!("{} {} {}", year, month, day);
		}

		(year, month, day) = next_date(year, month, day);
		weekday = (weekday + 1) % 7;
	}

	println!("{}", sundays_on_first_of_month);
}

fn
next_date
(
	year: i64,
	month: u8,
	day: u8,
)
-> (i64, u8, u8)
{
	let mut new_year = year;
	let mut new_month = month;
	let mut new_day = day+1;

	let mut is_end_of_month = false;
	if month == 1
	|| month == 3
	|| month == 5
	|| month == 7
	|| month == 8
	|| month == 10
	|| month == 12
	{
		if day == 31
		{
			is_end_of_month = true;
		}
	}

	if month == 2
	{
		if !is_leap_year(year) && day == 28
		||  is_leap_year(year) && day == 29
		{
			is_end_of_month = true;
		}
	}

	if month == 4
	|| month == 6
	|| month == 9
	|| month == 11
	{
		if day == 30
		{
			is_end_of_month = true;
		}
	}

	if is_end_of_month
	{
		new_month += 1;
		new_day = 1;

		if month == 12
		{
			new_year += 1;
			new_month = 1;
		}
	}

	return (new_year, new_month, new_day);
}


fn
is_twentieth_century
(
	year: i64,
	month: u8,
	day: u8,
)
-> bool
{
	year >= 1900 && year <= 2000
}

fn
is_leap_year
(
	year: i64
)
-> bool
{
	year % 100 != 0 && year % 4 == 0 || year % 400 == 0
}