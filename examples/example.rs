use proper_sort::{self, ProperString, cmp_ascii_ignore_case, error::Result};

fn main() -> Result<()> {
	let mut data = vec![
		"Adapter P.M. to P.M. 165mm to 175mm",
		"Adapter P.M. to P.M. 160mm to 180mm",
		"T-Shirt L Black",
		"T-Shirt XS Black",
		"T-Shirt Extra Large Black",
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

	data.sort_by(|a, b| proper_sort::compare(a, b));

	println!("{data:#?}");

	let a = "T-Shirt L Black";
	let parsed = ProperString::new(a);
	println!("{a}: {parsed:?}");
	
	let a = "T-Shirt XS Black";
	let parsed = ProperString::new(a);
	println!("{a}: {parsed:?}");
	
	let a = "T-Shirt Extra Large  Black";
	let parsed = ProperString::new(a);
	println!("{a}: {parsed:?}");
	
	Ok(())
}
