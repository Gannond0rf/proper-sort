use proper_sort::{self, ProperString, error::Result};

fn main() -> Result<()> {
	let mut data = vec![
		"T-Shirt L Black",
		"T-Shirt XS Black",
		"T-Shirt Extra Large Black",
		"T-Shirt Medium Black",
		"Crank 180mm Blue",
		"Crank 172.5mm Blue",
		"Crank 175mm Blue",
		"Crank 170mm Blue",
	];

	data.sort_by(|a, b| proper_sort::compare(a, b));
	
	assert_eq!(data, vec![
		"Crank 170mm Blue",
		"Crank 172.5mm Blue",
		"Crank 175mm Blue",
		"Crank 180mm Blue",
		"T-Shirt XS Black",
		"T-Shirt Medium Black",
		"T-Shirt L Black",
		"T-Shirt Extra Large Black",
	]);

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
