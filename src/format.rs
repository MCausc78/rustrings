//! Lightweight formatting module. Allows to justify and fill string.

use crate::converters::Base;
use crate::converters::Case;
use crate::converters::atoi_unsigned;
use crate::converters::itoa_signed;
use crate::converters::itoa_unsigned;

/// String alignment direction.
#[derive(Debug, Clone)]
pub enum FormatDirection {
	Left(usize),
	Center(usize),
	Right(usize),
	None,
}

/// Formatting options.
#[derive(Debug, Clone)]
pub struct FormatOptions {
	/// Aligment direction.
	pub padding: FormatDirection,
	
	/// Filling character.
	pub filling: Option<char>,
}

impl FormatOptions {
	/// Constructs FormatOptions.
	pub fn new(
		padding: FormatDirection,
		filling: Option<char>,
	) -> FormatOptions {
		FormatOptions {
			padding: padding,
			filling: filling,
		}
	}
	
	/// Returns padding options from `FormatOptions` object.
	pub fn get_padding(&self) -> &FormatDirection {
		&self.padding
	}
	
	/// Returns filling character from `FormatOptions` object.
	pub fn get_filling(&self) -> &Option<char> {
		&self.filling
	}
	
	/// Sets padding in object, and returns chain.
	pub fn set_padding(
		&mut self,
		padding: FormatDirection,
	) -> &mut FormatOptions {
		self.padding = padding;
		self
	}
	
	/// Sets filling character in object, and returns chain.
	pub fn set_filling(
		&mut self,
		filling: Option<char>,
	) -> &mut FormatOptions {
		self.filling = filling;
		self
	}
	
	/// Returns fields in object as tuple.
	pub fn get_fields(&self) -> (&FormatDirection, &Option<char>) {
		(&self.padding, &self.filling)
	}
	
	/// Sets fields in object, and returns chain.
	pub fn set_fields(
		&mut self,
		options: (FormatDirection, Option<char>),
	) -> &mut FormatOptions {
		self.padding = options.0;
		self.filling = options.1;
		self
	}
}

/// Formats the text.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::format::FormatDirection;
/// use rustrings::format::FormatOptions;
/// use rustrings::format::format_text;
/// 
/// assert_eq!(format_text(&"Hello, world!".to_string(),
///     FormatOptions::new(FormatDirection::Left(15), Some('_'))), "Hello, world!__");
/// assert_eq!(format_text(&"Hello, world!".to_string(),
///     FormatOptions::new(FormatDirection::Right(15), Some('_'))), "__Hello, world!");
/// ```
pub fn format_text(
	text: &String,
	options: FormatOptions,
) -> String {
	let filling = options
		.filling
		.unwrap_or(' ');
	let mut result = text.clone();
	match options.padding {
		FormatDirection::Left(width) => {
			let mut width = width;
			let length = text.len();
			if width < length {
				return result;
			}
			width -= length;
			result.push_str(&String::from(filling).repeat(width).as_str());
		},
		/* reserved */
		FormatDirection::Center(_width) => {
			
		},
		FormatDirection::Right(width) => {
			let mut width = width;
			let length = text.len();
			if width < length {
				return result;
			}
			width -= length;
			result.insert_str(0, String::from(filling)
				.repeat(width)
				.as_str());
		},
		_ => {},

	}
	result
}

/// Represents variadic argument for `c_format` function.
#[derive(Debug, Clone)]
pub enum CFormatArgument {
	/// Represents signed 8-bit integer.
	Int8(i8),
	
	/// Represents signed 16-bit integer.
	Int16(i16),
	
	/// Represents signed 32-bit integer.
	Int32(i32),
	
	/// Represents signed 64-bit integer.
	Int64(i64),
	
	/// Represents signed machine word.
	IntSize(isize),
	
	/// Represents unsigned 8-bit integer.
	UInt8(u8),
	
	/// Represents unsigned 16-bit integer.
	UInt16(u16),
	
	/// Represents unsigned 32-bit integer.
	UInt32(u32),
	
	/// Represents unsigned 64-bit integer.
	UInt64(u64),
	
	/// Represents unsigned machine word.
	UIntSize(usize),
	
	/// Represents character.
	Character(char),
	
	/// Represents string.
	String(String),
	
	/// Represents unsigned 8-bit integer.
	Pointer(*const u8),
}

/// Represents type specifier inside `c_format` function.
#[derive(Debug, Clone)]
pub enum CFormatTypeSpecifier {
	/// Represents decimal i32 format specifier. (`d`, `i`)
	Integer,
	
	/// Represents octal u32 format specifier. (`o`)
	Octal,
	
	/// Represents decimal u32 format specifier. (`u`)
	Unsigned,
	
	/// Represents hexdecimal u32 format specifier. Uses lower-case letters. (`x`)
	LowerHex,
	
	/// Represents hexdecimal u32 format specifier. Uses upper-case letters. (`X`)
	UpperHex,
	
	/// Represents character format specifier. (`c`)
	Character,
	
	/// Represents string format specifier. (`s`)
	String,
	
	/// Represents pointer format specifier. (`p`)
	Pointer,
	
	/// Represents escape of modulo operator (`%`) specifier. (`%`)
	Escape,
	
	/// Represents none of above specifiers.
	None,
}

/// Formats the string. Uses C-style formatting.
/// 
/// # Examples
/// 
/// ```
/// use rustrings::format::CFormatArgument;
/// use rustrings::format::c_format;
/// 
/// assert_eq!(c_format("*%d*".to_string(),
///     vec![
///         CFormatArgument::Int32(959),
///     ]
/// ), "*959*");
/// assert_eq!(c_format("*%2d*".to_string(),
///     vec![
///         CFormatArgument::Int32(959),
///     ]
/// ), "*959*");
/// assert_eq!(c_format("*%10d*".to_string(),
///     vec![
///         CFormatArgument::Int32(959),
///     ]
/// ), "*       959*");
/// assert_eq!(c_format("*%-10d*".to_string(),
///     vec![
///         CFormatArgument::Int32(959),
///     ]
/// ), "*959       *");
/// ```
pub fn c_format(
	format: String,
	arguments: Vec<CFormatArgument>,
) -> String {
	let mut result = String::new();
	let mut format_iterator = format.chars();
	let mut args_iterator = arguments.iter();
	while let Some(mut character) = format_iterator.next() {
		if character == '%' {
			character = if let Some(temp) = format_iterator.next() {
				temp
			} else {
				return result
			};
			let mut options = FormatOptions::new(FormatDirection::None, None);
			match character {
				'0' => {
					character = if let Some(temp) = format_iterator.next() {
						temp
					} else {
						return result
					};
					options.set_filling(Some('0'));
				}
				'*' => {
					character = if let Some(temp) = format_iterator.next() {
						temp
					} else {
						return result
					};
					if let CFormatArgument::IntSize(padding) = args_iterator.next().unwrap() {
						options.set_padding(if *padding < 0 {
							FormatDirection::Left(-*padding as usize)
						} else {
							FormatDirection::Right(*padding as usize)
						});
					} else {
						continue;
					}
				},
				'-' | '1'..='9' => {
					let sign = if character == '-' {
						character = if let Some(temp) = format_iterator.next() {
							temp
						} else {
							return result
						};
						true
					} else {
						false
					};
					let mut padding = String::new();
					loop {
						if !character.is_digit(10) {
							break;
						}
						padding.push(character);
						character = if let Some(temp) = format_iterator.next() {
							temp
						} else {
							return result
						};
					}
					options.set_padding(if sign {
						FormatDirection::Left(atoi_unsigned(&padding, Base::new(10), false).unwrap())
					} else {
						FormatDirection::Right(atoi_unsigned(&padding, Base::new(10), false).unwrap())
					});
				},
				_ => {},
			}
			let type_specifier = match character {
				'c' => CFormatTypeSpecifier::Character,
				'd' | 'i' => CFormatTypeSpecifier::Integer,
				'o' => CFormatTypeSpecifier::Octal,
				'u' => CFormatTypeSpecifier::Unsigned,
				'x' => CFormatTypeSpecifier::LowerHex,
				'X' => CFormatTypeSpecifier::UpperHex,
				'p' => CFormatTypeSpecifier::Pointer,
				's' => CFormatTypeSpecifier::String,
				'%' => CFormatTypeSpecifier::Escape,
				_ => CFormatTypeSpecifier::None,
			};
			match type_specifier {
				CFormatTypeSpecifier::Character => result.push(
					if let CFormatArgument::Character(temp) = args_iterator.next().unwrap() {
						*temp
					} else {
						return result
					}
				),
				CFormatTypeSpecifier::Integer => result.push_str(&format_text(&itoa_signed(
					if let CFormatArgument::Int32(temp) = args_iterator.next().unwrap() {
						*temp as isize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::Octal => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt32(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(8), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::Unsigned => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt32(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LowerHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt32(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::UpperHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt32(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Upper), options).as_str()),
				CFormatTypeSpecifier::Pointer => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::Pointer(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Upper),
						FormatOptions::new(FormatDirection::Right(usize::BITS as usize / u8::BITS as usize), Some('0'))).as_str()
					),
				CFormatTypeSpecifier::String => result.push_str(&format_text(&(
					if let CFormatArgument::String(temp) = args_iterator.next().unwrap() {
						temp.to_string()
					} else {
						return result
					}), options)),
				CFormatTypeSpecifier::Escape => result.push('%'),
				_ => {},
			}
			continue;
		}
		result.push(character);
	}
	result
}
