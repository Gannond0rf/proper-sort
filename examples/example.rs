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

	let test = "20mm";
	let token_string = ProperString::new(test);
	println!("{test}: {token_string:?}");

	let test = "2b";
	let token_string = ProperString::new(test);
	println!("{test}: {token_string:?}");

	let test = "172.5mm";
	let token_string = ProperString::new(test);
	println!("{test}: {token_string:?}");

	Ok(())
}
