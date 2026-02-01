use proper_sort::{self, ProperString, cmp_ascii_ignore_case, error::Result};

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

	data.sort_by(|a, b| proper_sort::compare(a, b));

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
	
	let test_1 = "b";
	let token_string_1 = ProperString::new(test_1);
	println!("{test_1}: {token_string_1:?}");
	
	let test_2 = "A";
	let token_string_2 = ProperString::new(test_2);
	println!("{test_2}: {token_string_2:?}");
	println!("{:?}", test_1.cmp(&test_2));

	println!("b, a");
	println!("{:?}", cmp_ascii_ignore_case("b", "a"));
	
	let a = "T-Shirt";
	let b = "Crank";
	println!("{a}, {b}");
	println!("{:?}", cmp_ascii_ignore_case(a, b));
	
	Ok(())
}
