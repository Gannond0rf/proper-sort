use std::str::FromStr;

use proper_sort::{self, proper_compare};
use proper_sort::{Result, TokenString};

fn main() -> Result<()> {
	
	let mut data = vec![
		"Adapter P.M. to P.M. 165mm to 175mm",
		"Adapter P.M. to P.M. 160mm to 180mm",
		"T-Shirt L Black",
		"T-Shirt XS Black",
		"T-Shirt Medium Black",
	];
	
	data.sort_by(|a, b| proper_compare(a, b));
	
	println!("{data:#?}");
	
	Ok(())
}