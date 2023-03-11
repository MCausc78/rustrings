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
	
	/// Represents decimal i8 format specifier. (`hhi`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hhi".to_string(), vec![CFormatArgument::Int8(42i8)]), "42");
	/// ```
	ByteInt,
	
	/// Represents hexadecimal u8 format specifier. Uses lower-case letters. (`hhx`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hhx".to_string(), vec![CFormatArgument::UInt8(42u8)]), "2a");
	/// ```
	ByteLowerHex,
	
	/// Represents hexadecimal u8 format specifier. Uses upper-case letters. (`hhX`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hhX".to_string(), vec![CFormatArgument::UInt8(42u8)]), "2A");
	/// ```
	ByteUpperHex,
	
	/// Represents octal u8 format specifier. (`hho`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hho".to_string(), vec![CFormatArgument::UInt8(42u8)]), "52");
	/// ```
	ByteOctal,
	
	/// Represents unsigned u8 format specifier. (`hhu`) 
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hhu".to_string(), vec![CFormatArgument::UInt8(142)]), "142");
	/// ```
	ByteUnsigned,
	
	/// Represents decimal i16 format specifier. (`hi`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hi".to_string(), vec![CFormatArgument::Int16(4242i16)]), "4242");
	/// ```
	ShortInt,
	
	/// Represents hexadecimal u16 format specifier. Uses lower-case letters. (`hx`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hx".to_string(), vec![CFormatArgument::UInt16(4242u16)]), "1092");
	/// assert_eq!(c_format("%hx".to_string(), vec![CFormatArgument::UInt16(42424u16)]), "a5b8");
	/// ```
	ShortLowerHex,
	
	/// Represents hexadecimal u16 format specifier. Uses upper-case letters. (`hX`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hX".to_string(), vec![CFormatArgument::UInt16(4242u16)]), "1092");
	/// assert_eq!(c_format("%hX".to_string(), vec![CFormatArgument::UInt16(42424u16)]), "A5B8");
	/// ```
	ShortUpperHex,
	
	/// Represents octal u16 format specifier. (`ho`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%ho".to_string(), vec![CFormatArgument::UInt16(4242u16)]), "10222");
	/// assert_eq!(c_format("%ho".to_string(), vec![CFormatArgument::UInt16(42424u16)]), "122670");
	/// ```
	ShortOctal,
	
	/// Represents decimal i16 format specifier. (`hu`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%hu".to_string(), vec![CFormatArgument::UInt16(4242u16)]), "4242");
	/// assert_eq!(c_format("%hu".to_string(), vec![CFormatArgument::UInt16(42424u16)]), "42424");
	/// ```
	ShortUnsigned,
	
	/// Represents decimal isize format specifier. (`li`, `ld`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%li".to_string(), vec![CFormatArgument::IntSize(6369051672839409isize)]), "6369051672839409");
	/// assert_eq!(c_format("%ld".to_string(), vec![CFormatArgument::IntSize(10465892101isize)]), "10465892101");
	/// ```
	LongInt,
	
	/// Represents hexadecimal usize format specifier. Uses lower-case letters. (`lx`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%lx".to_string(), vec![CFormatArgument::UIntSize(6369051672839409usize)]), "16a09e668404f1");
	/// assert_eq!(c_format("%lx".to_string(), vec![CFormatArgument::UIntSize(10465892101usize)]), "26fd0d705");
	/// ```
	LongLowerHex,
	
	/// Represents hexadecimal usize format specifier. Uses upper-case letters. (`lX`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%lX".to_string(), vec![CFormatArgument::UIntSize(6369051672839409usize)]), "16A09E668404F1");
	/// assert_eq!(c_format("%lX".to_string(), vec![CFormatArgument::UIntSize(10465892101usize)]), "26FD0D705");
	/// ```
	LongUpperHex,
	
	/// Represents octal usize format specifier. (`lo`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%lo".to_string(), vec![CFormatArgument::UIntSize(6369051672839409usize)]), "265011714641002361");
	/// assert_eq!(c_format("%lo".to_string(), vec![CFormatArgument::UIntSize(10465892101usize)]), "115764153405");
	/// ```
	LongOctal,
	
	/// Represents decimal usize format specifier. (`lu`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%lu".to_string(), vec![CFormatArgument::UIntSize(6369051672839409usize)]), "6369051672839409");
	/// assert_eq!(c_format("%lu".to_string(), vec![CFormatArgument::UIntSize(10465892101usize)]), "10465892101");
	/// ```
	LongUnsigned,
	
	/// Represents decimal i64 format specifier. (`lli`, `lld`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%lld".to_string(), vec![CFormatArgument::Int64(81985529216486895i64)]), "81985529216486895");
	/// assert_eq!(c_format("%lli".to_string(), vec![CFormatArgument::Int64(81985529216486895i64)]), "81985529216486895");
	/// ```
	LongLongInt,
	
	/// Represents hexadecimal u64 format specifier. Uses lower-case letters. (`llx`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%llx".to_string(), vec![CFormatArgument::UInt64(81985529216486895u64)]), "123456789abcdef");
	/// assert_eq!(c_format("%llx".to_string(), vec![CFormatArgument::UInt64(13464654573299691533u64)]), "badc0ffee0ddf00d");
	/// ```
	LongLongLowerHex,
	
	/// Represents hexadecimal u64 format specifier. Uses upper-case letters. (`llX`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%llX".to_string(), vec![CFormatArgument::UInt64(81985529216486895u64)]), "123456789ABCDEF");
	/// assert_eq!(c_format("%llX".to_string(), vec![CFormatArgument::UInt64(13464654573299691533u64)]), "BADC0FFEE0DDF00D");
	/// ```
	LongLongUpperHex,
	
	/// Represents octal u64 format specifier. (`llo`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%llo".to_string(), vec![CFormatArgument::UInt64(13464654573299691533u64)]), "1353340377734067370015");
	/// ```
	LongLongOctal,
	
	/// Represents decimal u64 format specifier. (`llu`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%llu".to_string(), vec![CFormatArgument::UInt64(13464654573299691533u64)]), "13464654573299691533");
	/// ```
	LongLongUnsigned,
	
	/// Represents string format specifier. (`s`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("hello, %s!".to_string(), vec![CFormatArgument::String("world".to_string())]), "hello, world!");
	/// ```
	String,
	
	/// Represents pointer format specifier. (`p`)
	Pointer,
	
	/// Represents escape of modulo operator (`%`) specifier. (`%`)
	/// 
	/// # Examples
	/// 
	/// ```
	/// use rustrings::format::CFormatArgument;
	/// use rustrings::format::c_format;
	/// 
	/// assert_eq!(c_format("%d%%".to_string(), vec![CFormatArgument::Int32(42)]), "42%");
	/// ```
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
				'h' => {
					character = if let Some(temp) = format_iterator.next() {
						temp
					} else {
						return result
					};
					match character {
						'h' => {
							character = if let Some(temp) = format_iterator.next() {
								temp
							} else {
								return result
							};
							match character {
								'd' | 'i' => CFormatTypeSpecifier::ByteInt,
								'x' => CFormatTypeSpecifier::ByteLowerHex,
								'X' => CFormatTypeSpecifier::ByteUpperHex,
								'u' => CFormatTypeSpecifier::ByteUnsigned,
								'o' => CFormatTypeSpecifier::ByteOctal,
								_ => {
									return result
								},
							}
						},
						'd' | 'i' => CFormatTypeSpecifier::ShortInt,
						'x' => CFormatTypeSpecifier::ShortLowerHex,
						'X' => CFormatTypeSpecifier::ShortUpperHex,
						'u' => CFormatTypeSpecifier::ShortUnsigned,
						'o' => CFormatTypeSpecifier::ShortOctal,
						_ => {
							return result
						},
					}
				},
				'l' => {
					character = if let Some(temp) = format_iterator.next() {
						temp
					} else {
						return result
					};
					match character {
						'l' => {
							character = if let Some(temp) = format_iterator.next() {
								temp
							} else {
								return result
							};
							match character {
								'd' | 'i' => CFormatTypeSpecifier::LongLongInt,
								'x' => CFormatTypeSpecifier::LongLongLowerHex,
								'X' => CFormatTypeSpecifier::LongLongUpperHex,
								'u' => CFormatTypeSpecifier::LongLongUnsigned,
								'o' => CFormatTypeSpecifier::LongLongOctal,
								_ => {
									return result
								},
							}
						},
						'd' | 'i' => CFormatTypeSpecifier::LongInt,
						'x' => CFormatTypeSpecifier::LongLowerHex,
						'X' => CFormatTypeSpecifier::LongUpperHex,
						'u' => CFormatTypeSpecifier::LongUnsigned,
						'o' => CFormatTypeSpecifier::LongOctal,
						_ => {
							return result
						},
					}
				},
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
				CFormatTypeSpecifier::LongInt => result.push_str(&format_text(&itoa_signed(
					if let CFormatArgument::IntSize(temp) = args_iterator.next().unwrap() {
						*temp as isize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongOctal => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UIntSize(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(8), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongUnsigned => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UIntSize(temp) = args_iterator.next().unwrap() {
						*temp
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongLowerHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UIntSize(temp) = args_iterator.next().unwrap() {
						*temp
					} else {
						return result
					}, Base::new(16), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongUpperHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UIntSize(temp) = args_iterator.next().unwrap() {
						*temp
					} else {
						return result
					}, Base::new(16), Case::Upper), options).as_str()),
				CFormatTypeSpecifier::LongLongInt => result.push_str(&format_text(&itoa_signed(
					if let CFormatArgument::Int64(temp) = args_iterator.next().unwrap() {
						*temp as isize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongLongOctal => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt64(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(8), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongLongUnsigned => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt64(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongLongLowerHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt64(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::LongLongUpperHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt64(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Upper), options).as_str()),
				CFormatTypeSpecifier::ShortInt => result.push_str(&format_text(&itoa_signed(
					if let CFormatArgument::Int16(temp) = args_iterator.next().unwrap() {
						*temp as isize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ShortOctal => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt16(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(8), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ShortUnsigned => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt16(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ShortLowerHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt16(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ShortUpperHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt16(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Upper), options).as_str()),
				CFormatTypeSpecifier::ByteInt => result.push_str(&format_text(&itoa_signed(
					if let CFormatArgument::Int8(temp) = args_iterator.next().unwrap() {
						*temp as isize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ByteOctal => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt8(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(8), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ByteUnsigned => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt8(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(10), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ByteLowerHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt8(temp) = args_iterator.next().unwrap() {
						*temp as usize
					} else {
						return result
					}, Base::new(16), Case::Lower), options).as_str()),
				CFormatTypeSpecifier::ByteUpperHex => result.push_str(&format_text(&itoa_unsigned(
					if let CFormatArgument::UInt8(temp) = args_iterator.next().unwrap() {
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
