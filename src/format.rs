//! Lightweight formatting module. Allows to justify and fill string.

/// String alignment direction.
pub enum FormatDirection {
	Left(usize),
	Center(usize),
	Right(usize),
	None,
}

/// Formatting options.
pub struct FormatOptions {
	pub padding: FormatDirection,
	pub filling: Option<char>,
}

impl FormatOptions {
	pub fn new(
		padding: FormatDirection,
		filling: Option<char>,
	) -> FormatOptions {
		FormatOptions {
			padding: padding,
			filling: filling,
		}
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
