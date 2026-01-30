use proper_sort::{self, ProperString, compare, error::Result};

//use crate::Result;

fn main() -> Result<()> {
	let mut data = vec![
		"Adapter P.M. to P.M. 165mm to 175mm",
		"Adapter P.M. to P.M. 160mm to 180mm",
		"T-Shirt L Black",
		"T-Shirt XS Black",
		"T-Shirt Medium Black",
		"Crank 180mm Blue",
		"Crank 172.5mm Blue",
		"Crank 175mm Blue",
		"Crank 170mm Blue",
		"A",
		"b2",
		"b1",
		"2b",
		"1b",
		"a",
		"48T",
		"36T",
		"20mm",
		"5mm",
		"30 mm",
		"10 mm",
	];

	data.sort_by(|a, b| compare(a, b));

	println!("{data:#?}");

	let token_string = ProperString::new("20mm");
	println!("{token_string:?}");

	let token_string = ProperString::new("2b");
	println!("{token_string:?}");

	let token_string = ProperString::new("172.5mm");
	println!("{token_string:?}");
	
	Ok(())
}
