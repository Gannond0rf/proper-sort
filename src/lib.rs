mod error;

use std::cmp::Ordering;
use rust_decimal::Decimal;

pub use error::*;

pub fn compare(a: &str, b: &str) -> std::cmp::Ordering {
	TokenString::from_str(a).cmp(&TokenString::from_str(b))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct TokenString {
	tokens: Vec<Token>,
}

impl TokenString {
	fn from_str(s: &str) -> Self {
		let chars: Vec<_> = s.chars().enumerate().collect();
		let mut tokens = Vec::new();
		
		if chars.len() == 1 {
			tokens.push(Token::new(&[chars[0].1].iter().collect::<String>()));
			return Self { tokens }
		}
		
		let mut changes = Vec::new();

		for window in chars.windows(2) {
			let prev = CharType::from(window[0].1);
			let curr = CharType::from(window[1].1);
			if window[1].0 == (chars.len() -1) {
				changes.push((window[1].0, prev))
			} else if curr != prev {
				changes.push((window[0].0, prev))
			};
		}

		let mut start_idx = 0;
		for (end_idx, _char_type) in changes {
			let text: String = chars[start_idx..=end_idx].iter().map(|(_,c)| c).collect();
			tokens.push(Token::new(&text));
			start_idx = end_idx + 1;
		}

		Self { tokens }
	}
}

impl Ord for TokenString {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let mut ord = Ordering::Equal;

		for (a, b) in self.tokens.iter().zip(other.tokens.iter()) {
			ord = a.cmp(b);
			if ord != Ordering::Equal {
				break;
			}
		}
		
		ord
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharType {
	Alpha,
	Number,
	Whitespace,
}

impl From<char> for CharType {
	fn from(value: char) -> Self {
		match value {
			c if is_char_alphabetic(c) => Self::Alpha,
			c if c.is_whitespace() => Self::Whitespace,
			c if c.is_numeric() => Self::Number,
			_ => Self::Alpha,
		}
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum Token {
	Word(String),
	Whitespace(String),
	Number((String, Decimal)),
	Size((String, Size)),
}

impl Token {
	pub fn new(value: &str) -> Token {
		let value_lower = value.to_lowercase();

		match is_whitespace(value) {
			true => return Token::Whitespace(value.to_string()),
			false => (),
		}

		match is_size(value_lower.as_str()) {
			Ok(size) => return Token::Size((value.to_string(), size)),
			Err(_) => (),
		}

		match is_number(value) {
			Ok(number) => return Token::Number((value.to_string(), number)),
			Err(_) => (),
		}

		Token::Word(value.to_lowercase())
	}
}

impl Ord for Token {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Token::Word(a), Token::Word(b)) => a.cmp(&b),
			(Token::Word(a), Token::Whitespace(b)) => a.cmp(&b),
			(Token::Word(a), Token::Number((b, _))) => a.cmp(&b),
			(Token::Word(a), Token::Size((b, _))) => a.cmp(&b),
			(Token::Whitespace(a), Token::Word(b)) => a.cmp(&b),
			(Token::Whitespace(a), Token::Whitespace(b)) => a.cmp(&b),
			(Token::Whitespace(a), Token::Number((b, _))) => a.cmp(&b),
			(Token::Whitespace(a), Token::Size((b, _))) => a.cmp(&b),
			(Token::Number((a, _)), Token::Word(b)) => a.cmp(&b),
			(Token::Number((a, _)), Token::Whitespace(b)) => a.cmp(&b),
			(Token::Number((_, na)), Token::Number((_, nb))) => na.cmp(&nb),
			(Token::Number((a, _)), Token::Size((b, _))) => a.cmp(&b),
			(Token::Size((a, _)), Token::Word(b)) => a.cmp(&b),
			(Token::Size((a, _)), Token::Whitespace(b)) => a.cmp(&b),
			(Token::Size((a, _)), Token::Number((b, _))) => a.cmp(&b),
			(Token::Size((_, sa)), Token::Size((_, sb))) => sa.cmp(&sb),
		}
	}
}

fn is_whitespace(value: &str) -> bool {
	value.chars().all(|c| c.is_whitespace())
}

fn is_number(value: &str) -> Result<Decimal> {
	match value.chars().all(|c| is_char_number(c)) {
		true => Ok(str::parse(&value.to_string().replace(",", ""))?),
		false => Err(Error::TokenNotNumber),
	}
}

fn is_size(value: &str) -> Result<Size> {
	Size::try_from(value)
}

fn is_char_alphabetic(c: char) -> bool {
	!c.is_whitespace() && !c.is_numeric()
}

fn is_char_number(c: char) -> bool {
	c.is_numeric() || c == ',' || c == '.'
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Size {
	Xxxxs,
	Xxxs,
	Xxs,
	Xs,
	S,
	SM,
	M,
	ML,
	L,
	Xl,
	Xxl,
	Xxxl,
	Xxxxl,
}

impl TryFrom<&str> for Size {
	type Error = crate::Error;

	fn try_from(value: &str) -> Result<Self> {
		match value {
			"xxxxs" => Ok(Size::Xxxxs),
			"xxxs" => Ok(Size::Xxxs),
			"xxs" => Ok(Size::Xxs),
			"xs" => Ok(Size::Xs),
			"s" => Ok(Size::S),
			"sm" => Ok(Size::SM),
			"s/m" => Ok(Size::SM),
			"s-m" => Ok(Size::SM),
			"m" => Ok(Size::M),
			"ml" => Ok(Size::ML),
			"m/l" => Ok(Size::ML),
			"m-l" => Ok(Size::ML),
			"l" => Ok(Size::L),
			"xl" => Ok(Size::Xl),
			"xxl" => Ok(Size::Xxl),
			"xxxl" => Ok(Size::Xxxl),
			"xxxxl" => Ok(Size::Xxxxl),
			"small" => Ok(Size::S),
			"medium" => Ok(Size::M),
			"med" => Ok(Size::M),
			"large" => Ok(Size::L),
			"extra small" => Ok(Size::Xs),
			"x-small" => Ok(Size::Xs),
			"xx-small" => Ok(Size::Xxs),
			"xxx-small" => Ok(Size::Xxxs),
			"xxxx-small" => Ok(Size::Xxxxs),
			"extra large" => Ok(Size::Xl),
			"x-large" => Ok(Size::Xl),
			"xx-large" => Ok(Size::Xxl),
			"xxx-large" => Ok(Size::Xxxl),
			"xxxx-large" => Ok(Size::Xxxxl),
			_ => Err(Error::TokenNotSize),
		}
	}
}
