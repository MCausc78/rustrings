//! String to integer/integer to String operations.
//! 
//! This module contains converters to convert numbers to strings and strings to numbers.

/// Number base representation.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::converters::{Base, Case, itoa_signed};
/// 
/// assert_eq!(itoa_signed(12345, Base::new(2), Case::Lower), "11000000111001");
/// assert_eq!(itoa_signed(12345, Base::new(8), Case::Lower), "30071");
/// assert_eq!(itoa_signed(12345, Base::new(10), Case::Lower), "12345");
/// assert_eq!(itoa_signed(12345, Base::new(16), Case::Lower), "3039");
/// assert_eq!(itoa_signed(12345, Base::new(16), Case::Upper), "3039");
/// // assert_ne!(itoa_signed(12345, Base::new(1), Case::Lower), "?"); // panics, base is less than 2
/// // assert_ne!(itoa_signed(12345, Base::new(37), Case::Lower), "?"); // panics, base is greater than 36
/// ```
#[derive(Debug, PartialEq)]
pub struct Base(i32);

impl Base {
	/// Safely constructs base.
	/// Panics if `base` not in `2..36` range.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::converters::Base;
	/// 
	/// assert_eq!(Base::new(2), Base::unsafe_new(2));
	/// assert_eq!(Base::new(8), Base::unsafe_new(8));
	/// assert_eq!(Base::new(10), Base::unsafe_new(10));
	/// assert_eq!(Base::new(16), Base::unsafe_new(16));
	/// assert_eq!(Base::new(32), Base::unsafe_new(32));
	/// assert_eq!(Base::new(36), Base::unsafe_new(36));
	/// // let _ = Base::new(1); // panics
	/// // let _ = Base::new(37); // panics
	/// ```
	pub fn new(base: i32) -> Base {
		if !(2..=36).contains(&base) {
			panic!("base not in 2..36 range ({})", base);
		}
		Base(base)
	}
	
	/// Unsafely constructs base.
	/// Does not panics if `base` not in `2..36` range.
	pub fn unsafe_new(base: i32) -> Base {
		Base(base)
	}
}

/// ASCII case representation.
#[derive(Debug)]
pub enum Case {
	/// Represents lower-case letters.
	Lower,
	
	/// Represents upper-case letters.
	Upper,
}

impl Case {
	/// Returns alphabet. Letters depends on case.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::converters::Case;
	/// 
	/// assert_eq!(Case::Lower.alphabet(), "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>());
	/// assert_eq!(Case::Upper.alphabet(), "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>());
	/// ```
	pub fn alphabet(&self) -> Vec<char> {
		match self {
			Case::Lower => "0123456789abcdefghijklmnopqrstuvwxyz",
			Case::Upper => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		}
			.chars()
			.collect::<Vec<char>>()
	}
}

/// Converts signed integers into Strings.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::converters::{Base, Case, itoa_signed};
/// 
/// assert_eq!(itoa_signed(0, Base::new(10), Case::Lower), "0");
/// assert_eq!(itoa_signed(1, Base::new(10), Case::Lower), "1");
/// assert_eq!(itoa_signed(10, Base::new(10), Case::Lower), "10");
/// assert_eq!(itoa_signed(100, Base::new(10), Case::Lower), "100");
/// assert_eq!(itoa_signed(1000, Base::new(10), Case::Lower), "1000");
/// assert_eq!(itoa_signed(10000, Base::new(10), Case::Lower), "10000");
/// assert_eq!(itoa_signed(-1, Base::new(10), Case::Lower), "-1");
/// assert_eq!(itoa_signed(-10, Base::new(10), Case::Lower), "-10");
/// assert_eq!(itoa_signed(-100, Base::new(10), Case::Lower), "-100");
/// assert_eq!(itoa_signed(-1000, Base::new(10), Case::Lower), "-1000");
/// assert_eq!(itoa_signed(-10000, Base::new(10), Case::Lower), "-10000");
/// ```
pub fn itoa_signed(number: isize, base: Base, case: Case) -> String {
	let mut number = number;
	let base = base.0;
	let mut result = String::new();
	let sign = if number < 0isize {
		number = -number;
		true
	} else {
		false
	};
	let alphabet = case.alphabet();
	loop {
		result.push(alphabet[(number % base as isize) as usize]);
		number /= base as isize;
		if number == 0 {
			break;
		}
	}
	if sign {
		result.push('-');
	}
	result
		.chars()
		.rev()
		.collect()
}

/// Converts unsigned integers into Strings.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::converters::Base;
/// use rustrings::converters::Case;
/// use rustrings::converters::itoa_unsigned;
/// 
/// assert_eq!(itoa_unsigned(0, Base::new(10), Case::Lower), "0");
/// assert_eq!(itoa_unsigned(1, Base::new(10), Case::Lower), "1");
/// assert_eq!(itoa_unsigned(10, Base::new(10), Case::Lower), "10");
/// assert_eq!(itoa_unsigned(100, Base::new(10), Case::Lower), "100");
/// assert_eq!(itoa_unsigned(1000, Base::new(10), Case::Lower), "1000");
/// assert_eq!(itoa_unsigned(10000, Base::new(10), Case::Lower), "10000");
/// assert_eq!(itoa_unsigned(isize::MAX as usize, Base::new(10), Case::Lower), "9223372036854775807");
/// assert_eq!(itoa_unsigned(usize::MAX, Base::new(10), Case::Lower), "18446744073709551615");
/// assert_eq!(itoa_unsigned(isize::MAX as usize, Base::new(16), Case::Lower), "7fffffffffffffff");
/// assert_eq!(itoa_unsigned(usize::MAX, Base::new(16), Case::Lower), "ffffffffffffffff");
/// ```
pub fn itoa_unsigned(number: usize, base: Base, case: Case) -> String {
	let mut number = number;
	let base = base.0;
	let mut result = String::new();
	let alphabet = case.alphabet();
	loop {
		result.push(alphabet[number % base as usize]);
		number /= base as usize;
		if number == 0 {
			break;
		}
	}
	result
		.chars()
		.rev()
		.collect()
}

/// Returned by `atoi_signed` and `atoi_unsigned` if parsing of string failed.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseIntKind {
	/// Empty string.
	Empty,
	
	/// Invalid character in the start of string.
	InvalidCharacter,
	
	/// Overflow.
	Overflow,
}

/// Converts string to number with sign, returning `Ok(isize)` if successfully parsed, otherwise returns `Err(ParseIntKind)` on error.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::converters::Base;
/// use rustrings::converters::atoi_signed;
/// 
/// assert_eq!(atoi_signed(&"0".to_string(), Base::new(2), false), Ok(0));
/// assert_eq!(atoi_signed(&"11111111".to_string(), Base::new(2), false), Ok(255));
/// assert_eq!(atoi_signed(&"1111111111111111".to_string(), Base::new(2), false), Ok(65535));
/// assert_eq!(atoi_signed(&"377".to_string(), Base::new(8), false), Ok(255));
/// assert_eq!(atoi_signed(&"255".to_string(), Base::new(10), false), Ok(255));
/// assert_eq!(atoi_signed(&"ff".to_string(), Base::new(16), false), Ok(255));
/// assert_eq!(atoi_signed(&"-11111111".to_string(), Base::new(2), false), Ok(-255));
/// assert_eq!(atoi_signed(&"-1111111111111111".to_string(), Base::new(2), false), Ok(-65535));
/// assert_eq!(atoi_signed(&"-377".to_string(), Base::new(8), false), Ok(-255));
/// assert_eq!(atoi_signed(&"-255".to_string(), Base::new(10), false), Ok(-255));
/// assert_eq!(atoi_signed(&"-ff".to_string(), Base::new(16), false), Ok(-255));
/// ```
pub fn atoi_signed(
	o: &String,
	base: Base,
	ignore_overflow: bool,
) -> Result<isize, ParseIntKind> {

	let base = base.0;
	let mut iterator = o.chars();
	let mut success = false;
	let mut sign = false;
	let mut alphabet = Case::Upper.alphabet();
	alphabet.truncate(base as usize);
	let mut looped = false;
	let mut result = 0isize;
	loop {
		let char = iterator.next();
		if char.is_none() {
			break;
		}
		let char = char
			.unwrap()
			.to_ascii_uppercase();
		if char == '-' {
			sign = true;
			continue;
		}
		if char == '\t' ||
			char == '\n' ||
			char == '\x0b' ||
			char == '\x0c' ||
			char == '\n' ||
			char == ' ' ||
			char == '_'
		{
			continue;
		}
		looped = true;
		let position = alphabet
			.iter()
			.position(|&e| e == char);
		if position.is_none() {
			break;
		}
		success = true;
		let mut tmp = result.overflowing_mul(base as isize);
		if !ignore_overflow && tmp.1 {
			return Err(ParseIntKind::Overflow);
		}
		result = tmp.0;
		tmp = result.overflowing_add(position.unwrap() as isize);
		if !ignore_overflow && tmp.1 {
			return Err(ParseIntKind::Overflow);
		}
		result = tmp.0;
	}
	if success {
		Ok(if sign {
			-result
		} else {
			result
		})
	} else {
		Err(if looped {
			ParseIntKind::InvalidCharacter
		} else {
			ParseIntKind::Empty
		})
	}
}

/// Converts string to number without sign, returning `Ok(usize)` if successfully parsed, otherwise returns `Err(ParseIntKind)` on error.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::converters::Base;
/// use rustrings::converters::atoi_unsigned;
/// 
/// assert_eq!(atoi_unsigned(&"0".to_string(), Base::new(2), false), Ok(0));
/// assert_eq!(atoi_unsigned(&"11111111".to_string(), Base::new(2), false), Ok(255));
/// assert_eq!(atoi_unsigned(&"1111111111111111".to_string(), Base::new(2), false), Ok(65535));
/// assert_eq!(atoi_unsigned(&"377".to_string(), Base::new(8), false), Ok(255));
/// assert_eq!(atoi_unsigned(&"255".to_string(), Base::new(10), false), Ok(255));
/// assert_eq!(atoi_unsigned(&"7f".to_string(), Base::new(16), false), Ok(127));
/// assert_eq!(atoi_unsigned(&"ff".to_string(), Base::new(16), false), Ok(255));
/// assert_eq!(atoi_unsigned(&"7fff".to_string(), Base::new(16), false), Ok(32767));
/// assert_eq!(atoi_unsigned(&"ffff".to_string(), Base::new(16), false), Ok(65535));
/// assert_eq!(atoi_unsigned(&"7fffffff".to_string(), Base::new(16), false), Ok(2147483647));
/// assert_eq!(atoi_unsigned(&"ffffffff".to_string(), Base::new(16), false), Ok(4294967295));
/// assert_eq!(atoi_unsigned(&"7fffffffffffffff".to_string(), Base::new(16), false), Ok(9223372036854775807));
/// assert_eq!(atoi_unsigned(&"ffffffffffffffff".to_string(), Base::new(16), false), Ok(18446744073709551615));
/// ```
pub fn atoi_unsigned(
	o: &String,
	base: Base,
	ignore_overflow: bool,
) -> Result<usize, ParseIntKind> {
	
	let base = base.0;
	let mut iterator = o.chars();
	let mut success = false;
	let mut alphabet = Case::Upper.alphabet();
	alphabet.truncate(base as usize);
	let mut looped = false;
	let mut result = 0usize;
	loop {
		let char = iterator.next();
		if char.is_none() {
			break;
		}
		let char = char
			.unwrap()
			.to_ascii_uppercase();
		if char == '\t' ||
			char == '\n' ||
			char == '\x0b' ||
			char == '\x0c' ||
			char == '\n' ||
			char == ' ' ||
			char == '_'
		{
			continue;
		}
		looped = true;
		let position = alphabet
			.iter()
			.position(|&e| e == char);
		if position.is_none() {
			break;
		}
		success = true;
		let mut tmp = result.overflowing_mul(base as usize);
		if !ignore_overflow && tmp.1 {
			return Err(ParseIntKind::Overflow);
		}
		result = tmp.0;
		tmp = result.overflowing_add(position.unwrap() as usize);
		if !ignore_overflow && tmp.1 {
			return Err(ParseIntKind::Overflow);
		}
		result = tmp.0;
	}
	if success {
		Ok(result)
	} else {
		Err(if looped {
			ParseIntKind::InvalidCharacter
		} else {
			ParseIntKind::Empty
		})
	}
}

/// Same as C `atoi`.
/// For some reason, you need `use rustrings::converters::{Base, atoi_signed};`.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::converters::{Base, atoi_signed};
/// use rustrings::converters::atoi;
/// 
/// assert_eq!(atoi!(&"1234".to_string()), 1234);
/// assert_eq!(atoi!(&"12345".to_string()), 12345);
/// assert_eq!(atoi!(&"1234abc".to_string()), 1234);
/// assert_eq!(atoi!(&"a1b2".to_string()), 0);
/// ```
#[macro_export]
macro_rules! atoi {
	($e:expr) => {
		{		
			let tmp = atoi_signed($e, Base::new(10), true).unwrap_or(0);
			if (tmp & 0xffffffffisize) != tmp {
				0
			} else {
				tmp
			}
		}
	};
}

pub use atoi;
