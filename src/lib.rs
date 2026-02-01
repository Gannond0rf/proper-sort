pub mod error;
use std::cmp::Ordering;

pub use error::*;

use crate::{Result, Error};

pub fn compare(a: &str, b: &str) -> std::cmp::Ordering {
	ProperString::new(a).cmp(&ProperString::new(b))
}

pub fn cmp_ascii_ignore_case(a: &str, b: &str) -> Ordering {
	if a == b { return Ordering::Equal }
	
	for (a, b) in a.as_bytes().iter().zip(b.as_bytes().iter()).map(|(a, b)| (*a, *b)) {
		if a == b { continue };
		if is_ascii_upper(a) && is_ascii_lower(b) {
			match a.cmp(&(b - 32)) {
				Ordering::Equal => continue,
				ord => return ord,
			}
		} else if is_ascii_lower(a) && is_ascii_upper(b) {
			match a.cmp(&(b + 32)) {
				Ordering::Equal => continue,
				ord => return ord,
			}
		} else {
			match a.cmp(&b) {
				Ordering::Equal => continue,
				ord => return ord,
			}
		}
	}
	
	match a.len().cmp(&b.len()) {
		Ordering::Equal => a.cmp(&b),
		ord => ord,
	}
}

fn is_ascii_upper(b: u8) -> bool {
	b > 64 && b < 91
}

fn is_ascii_lower(b: u8) -> bool {
	b > 96 && b < 123
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct ProperString<'a> {
	pub tokens: Vec<Token<'a>>
}

impl<'a> ProperString<'a> {
	pub fn new(input: &'a str) -> Self {
		let mut tokens: Vec<Token> = Vec::new();
		let mut boundaries = vec![0];
		input.as_bytes().iter().enumerate()
			.filter_map(|(i, b)| b.is_ascii_whitespace().then(|| i))
			.for_each(|i| boundaries.push(i));
		
		boundaries.push(input.len());
		
		for bp in boundaries.windows(2) {
			let s = if bp[0] == 0 { 0 } else { bp[0] + 1 };
			let w = &input[s..bp[1]];
			
			if w.len() == 0 { continue };
			
			if let Some(size) = Size::try_from(w).ok() {
				tokens.push(Token::Size(w, size));
				continue;
			}
			
			if let Some(num) = w.parse().ok() {
				tokens.push(Token::Number(w, num));
				continue;
			}
			
			let mut num_bounds = vec![0];
			let wb = w.as_bytes();
			for i in 0..w.len() - 1 {
				let changed = (wb[i].is_ascii_digit() && !wb[i + 1].is_ascii_digit()) || (!wb[i].is_ascii_digit() && wb[i + 1].is_ascii_digit());
				if changed { num_bounds.push(i) }
			}
			
			if num_bounds.len() == 1 {
				tokens.push(Token::Text(w));
				continue;
			}
			
			num_bounds.push(w.len() - 1);
			
			for wbp in num_bounds.windows(2) {
				let s = if wbp[0] == 0 { 0 } else { wbp[0] + 1 };
				let nw = &w[s..=wbp[1]];
				
				if let Some(num) = nw.parse().ok() {
					tokens.push(Token::Number(nw, num));
				} else {
					tokens.push(Token::Text(nw));
				}
			}
		}

		Self { tokens }
	}
}

impl Ord for ProperString<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		for (a, b) in self.tokens.iter().zip(other.tokens.iter()) {
			let ord = a.cmp(b);
			match ord == Ordering::Equal {
				true => continue,
				false => return ord,
			}
		}
		
		self.tokens.len().cmp(&other.tokens.len())
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum Token<'a> {
	Text(&'a str),
	Number(&'a str, i64),
	Size(&'a str, Size),
}

impl Ord for Token<'_> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Token::Text(a), Token::Text(b)) => cmp_ascii_ignore_case(a, b),
			(Token::Text(a), Token::Number(b, _)) => a.cmp(b),
			(Token::Text(a), Token::Size(b, _)) => a.cmp(b),
			(Token::Number(a, _), Token::Text(b)) => a.cmp(b),
			(Token::Number(_, a), Token::Number(_, b)) => a.cmp(b),
			(Token::Number(a, _), Token::Size(b, _)) => a.cmp(b),
			(Token::Size(a, _), Token::Text(b)) => a.cmp(b),
			(Token::Size(a, _), Token::Number(b, _)) => a.cmp(b),
			(Token::Size(_, a), Token::Size(_, b)) => a.cmp(b),
		}
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Size {
	XXXXS,
	XXXS,
	XXS,
	XS,
	S,
	SM,
	M,
	ML,
	L,
	LXL,
	XL,
	XXL,
	XXXL,
	XXXXL,
}

impl TryFrom<&str> for Size {
	type Error = crate::Error;

	fn try_from(value: &str) -> Result<Self> {
		match value {
			val if val.eq_ignore_ascii_case("xxxxs") => Ok(Size::XXXXS),
			val if val.eq_ignore_ascii_case("xxxs") => Ok(Size::XXXS),
			val if val.eq_ignore_ascii_case("xxs") => Ok(Size::XXS),
			val if val.eq_ignore_ascii_case("xs") => Ok(Size::XS),
			val if val.eq_ignore_ascii_case("s") => Ok(Size::S),
			val if val.eq_ignore_ascii_case("sm") => Ok(Size::SM),
			val if val.eq_ignore_ascii_case("s/m") => Ok(Size::SM),
			val if val.eq_ignore_ascii_case("s-m") => Ok(Size::SM),
			val if val.eq_ignore_ascii_case("m") => Ok(Size::M),
			val if val.eq_ignore_ascii_case("ml") => Ok(Size::ML),
			val if val.eq_ignore_ascii_case("m/l") => Ok(Size::ML),
			val if val.eq_ignore_ascii_case("m-l") => Ok(Size::ML),
			val if val.eq_ignore_ascii_case("l") => Ok(Size::L),
			val if val.eq_ignore_ascii_case("lxl") => Ok(Size::LXL),
			val if val.eq_ignore_ascii_case("l/xl") => Ok(Size::LXL),
			val if val.eq_ignore_ascii_case("l-xl") => Ok(Size::LXL),
			val if val.eq_ignore_ascii_case("xl") => Ok(Size::XL),
			val if val.eq_ignore_ascii_case("xxl") => Ok(Size::XXL),
			val if val.eq_ignore_ascii_case("xxxl") => Ok(Size::XXXL),
			val if val.eq_ignore_ascii_case("xxxxl") => Ok(Size::XXXXL),
			val if val.eq_ignore_ascii_case("small") => Ok(Size::S),
			val if val.eq_ignore_ascii_case("medium") => Ok(Size::M),
			val if val.eq_ignore_ascii_case("med") => Ok(Size::M),
			val if val.eq_ignore_ascii_case("large") => Ok(Size::L),
			val if val.eq_ignore_ascii_case("extra small") => Ok(Size::XS),
			val if val.eq_ignore_ascii_case("x-small") => Ok(Size::XS),
			val if val.eq_ignore_ascii_case("xx-small") => Ok(Size::XXS),
			val if val.eq_ignore_ascii_case("xxx-small") => Ok(Size::XXXS),
			val if val.eq_ignore_ascii_case("xxxx-small") => Ok(Size::XXXXS),
			val if val.eq_ignore_ascii_case("extra large") => Ok(Size::XL),
			val if val.eq_ignore_ascii_case("x-large") => Ok(Size::XL),
			val if val.eq_ignore_ascii_case("xx-large") => Ok(Size::XXL),
			val if val.eq_ignore_ascii_case("xxx-large") => Ok(Size::XXXL),
			val if val.eq_ignore_ascii_case("xxxx-large") => Ok(Size::XXXXL),
			_ => Err(Error::TokenNotSize),
		}
	}
}

impl TryFrom<&[u8]> for Size {
	type Error = crate::Error;

	fn try_from(value: &[u8]) -> Result<Self> {
		match value {
			val if val.eq_ignore_ascii_case(b"xxxxs") => Ok(Size::XXXXS),
			val if val.eq_ignore_ascii_case(b"xxxs") => Ok(Size::XXXS),
			val if val.eq_ignore_ascii_case(b"xxs") => Ok(Size::XXS),
			val if val.eq_ignore_ascii_case(b"xs") => Ok(Size::XS),
			val if val.eq_ignore_ascii_case(b"s") => Ok(Size::S),
			val if val.eq_ignore_ascii_case(b"sm") => Ok(Size::SM),
			val if val.eq_ignore_ascii_case(b"s/m") => Ok(Size::SM),
			val if val.eq_ignore_ascii_case(b"s-m") => Ok(Size::SM),
			val if val.eq_ignore_ascii_case(b"m") => Ok(Size::M),
			val if val.eq_ignore_ascii_case(b"ml") => Ok(Size::ML),
			val if val.eq_ignore_ascii_case(b"m/l") => Ok(Size::ML),
			val if val.eq_ignore_ascii_case(b"m-l") => Ok(Size::ML),
			val if val.eq_ignore_ascii_case(b"l") => Ok(Size::L),
			val if val.eq_ignore_ascii_case(b"xl") => Ok(Size::XL),
			val if val.eq_ignore_ascii_case(b"xxl") => Ok(Size::XXL),
			val if val.eq_ignore_ascii_case(b"xxxl") => Ok(Size::XXXL),
			val if val.eq_ignore_ascii_case(b"xxxxl") => Ok(Size::XXXXL),
			val if val.eq_ignore_ascii_case(b"small") => Ok(Size::S),
			val if val.eq_ignore_ascii_case(b"medium") => Ok(Size::M),
			val if val.eq_ignore_ascii_case(b"med") => Ok(Size::M),
			val if val.eq_ignore_ascii_case(b"large") => Ok(Size::L),
			val if val.eq_ignore_ascii_case(b"extra small") => Ok(Size::XS),
			val if val.eq_ignore_ascii_case(b"x-small") => Ok(Size::XS),
			val if val.eq_ignore_ascii_case(b"xx-small") => Ok(Size::XXS),
			val if val.eq_ignore_ascii_case(b"xxx-small") => Ok(Size::XXXS),
			val if val.eq_ignore_ascii_case(b"xxxx-small") => Ok(Size::XXXXS),
			val if val.eq_ignore_ascii_case(b"extra large") => Ok(Size::XL),
			val if val.eq_ignore_ascii_case(b"x-large") => Ok(Size::XL),
			val if val.eq_ignore_ascii_case(b"xx-large") => Ok(Size::XXL),
			val if val.eq_ignore_ascii_case(b"xxx-large") => Ok(Size::XXXL),
			val if val.eq_ignore_ascii_case(b"xxxx-large") => Ok(Size::XXXXL),
			_ => Err(Error::TokenNotSize),
		}
	}
}
