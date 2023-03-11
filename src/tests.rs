#[cfg(test)]
mod tests {
	use crate::converters::*;
	use crate::format::*;

	#[test]
	fn test_itoa_hex_speak() {
		assert_eq!(itoa_signed(0x8BADF00D, Base::new(16), Case::Lower), "8badf00d");
		assert_eq!(itoa_signed(0x8BADF00D, Base::new(16), Case::Upper), "8BADF00D");
		
		assert_eq!(itoa_signed(0xABADBABE, Base::new(16), Case::Lower), "abadbabe");
		assert_eq!(itoa_signed(0xABADBABE, Base::new(16), Case::Upper), "ABADBABE");
		
		assert_eq!(itoa_signed(0x1BADB002, Base::new(16), Case::Lower), "1badb002");
		assert_eq!(itoa_signed(0x1BADB002, Base::new(16), Case::Upper), "1BADB002");
		
		assert_eq!(itoa_signed(0xBAADF00D, Base::new(16), Case::Lower), "baadf00d");
		assert_eq!(itoa_signed(0xBAADF00D, Base::new(16), Case::Upper), "BAADF00D");
		
		assert_eq!(itoa_signed(0xBADCAB1E, Base::new(16), Case::Lower), "badcab1e");
		assert_eq!(itoa_signed(0xBADCAB1E, Base::new(16), Case::Upper), "BADCAB1E");
		
		assert_eq!(itoa_signed(0xBADDCAFE, Base::new(16), Case::Lower), "baddcafe");
		assert_eq!(itoa_signed(0xBADDCAFE, Base::new(16), Case::Upper), "BADDCAFE");
		
		assert_eq!(itoa_signed(0xBEADFACE, Base::new(16), Case::Lower), "beadface");
		assert_eq!(itoa_signed(0xBEADFACE, Base::new(16), Case::Upper), "BEADFACE");
		
		assert_eq!(itoa_signed(0xCAFEBABE, Base::new(16), Case::Lower), "cafebabe");
		assert_eq!(itoa_signed(0xCAFEBABE, Base::new(16), Case::Upper), "CAFEBABE");
		
		assert_eq!(itoa_signed(0xD15EA5E, Base::new(16), Case::Lower), "d15ea5e");
		assert_eq!(itoa_signed(0xD15EA5E, Base::new(16), Case::Upper), "D15EA5E");
		
		assert_eq!(itoa_signed(0xDEADBABE, Base::new(16), Case::Lower), "deadbabe");
		assert_eq!(itoa_signed(0xDEADBABE, Base::new(16), Case::Upper), "DEADBABE");
		
		assert_eq!(itoa_signed(0xDEADBEEF, Base::new(16), Case::Lower), "deadbeef");
		assert_eq!(itoa_signed(0xDEADBEEF, Base::new(16), Case::Upper), "DEADBEEF");
		
		assert_eq!(itoa_signed(0xDEADDEAD, Base::new(16), Case::Lower), "deaddead");
		assert_eq!(itoa_signed(0xDEADDEAD, Base::new(16), Case::Upper), "DEADDEAD");
		
		assert_eq!(itoa_signed(0xDEADFA11, Base::new(16), Case::Lower), "deadfa11");
		assert_eq!(itoa_signed(0xDEADFA11, Base::new(16), Case::Upper), "DEADFA11");
		
		assert_eq!(itoa_signed(0xDEFEC8ED, Base::new(16), Case::Lower), "defec8ed");
		assert_eq!(itoa_signed(0xDEFEC8ED, Base::new(16), Case::Upper), "DEFEC8ED");
		
		assert_eq!(itoa_signed(0xFACEFEED, Base::new(16), Case::Lower), "facefeed");
		assert_eq!(itoa_signed(0xFACEFEED, Base::new(16), Case::Upper), "FACEFEED");
		
		assert_eq!(itoa_signed(0xFEE1DEAD, Base::new(16), Case::Lower), "fee1dead");
		assert_eq!(itoa_signed(0xFEE1DEAD, Base::new(16), Case::Upper), "FEE1DEAD");
		
		assert_eq!(itoa_signed(0xFEEDCAFE, Base::new(16), Case::Lower), "feedcafe");
		assert_eq!(itoa_signed(0xFEEDCAFE, Base::new(16), Case::Upper), "FEEDCAFE");
		
		assert_eq!(itoa_signed(0xC0FFEE, Base::new(16), Case::Lower), "c0ffee");
		assert_eq!(itoa_signed(0xC0FFEE, Base::new(16), Case::Upper), "C0FFEE");
		
		assert_eq!(itoa_signed(0xE011CFD0, Base::new(16), Case::Lower), "e011cfd0");
		assert_eq!(itoa_signed(0xE011CFD0, Base::new(16), Case::Upper), "E011CFD0");
		
		assert_eq!(itoa_signed(0xFACE8D, Base::new(16), Case::Lower), "face8d");
		assert_eq!(itoa_signed(0xFACE8D, Base::new(16), Case::Upper), "FACE8D");
		
		assert_eq!(itoa_signed(0xFEEE, Base::new(16), Case::Lower), "feee");
		assert_eq!(itoa_signed(0xFEEE, Base::new(16), Case::Upper), "FEEE");
		
		assert_eq!(itoa_signed(0xCCCCCCCC, Base::new(16), Case::Lower), "cccccccc");
		assert_eq!(itoa_signed(0xCCCCCCCC, Base::new(16), Case::Upper), "CCCCCCCC");
	}

	#[test]
	fn test_atoi_hex_speak() {
		assert_eq!(atoi_signed(&"8badf00d".to_string(), Base::new(16), false), Ok(0x8BADF00D));
		assert_eq!(atoi_signed(&"abadbabe".to_string(), Base::new(16), false), Ok(0xABADBABE));
		assert_eq!(atoi_signed(&"1badb002".to_string(), Base::new(16), false), Ok(0x1BADB002));
		assert_eq!(atoi_signed(&"baadf00d".to_string(), Base::new(16), false), Ok(0xBAADF00D));
		assert_eq!(atoi_signed(&"badcab1e".to_string(), Base::new(16), false), Ok(0xBADCAB1E));
		assert_eq!(atoi_signed(&"baddcafe".to_string(), Base::new(16), false), Ok(0xBADDCAFE));
		assert_eq!(atoi_signed(&"beadface".to_string(), Base::new(16), false), Ok(0xBEADFACE));
		assert_eq!(atoi_signed(&"cafebabe".to_string(), Base::new(16), false), Ok(0xCAFEBABE));
		assert_eq!(atoi_signed(&"d15ea5e".to_string(), Base::new(16), false), Ok(0xD15EA5E));
		assert_eq!(atoi_signed(&"deadbabe".to_string(), Base::new(16), false), Ok(0xDEADBABE));
		assert_eq!(atoi_signed(&"deadbeef".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_signed(&"deaddead".to_string(), Base::new(16), false), Ok(0xDEADDEAD));
		assert_eq!(atoi_signed(&"deadfa11".to_string(), Base::new(16), false), Ok(0xDEADFA11));
		assert_eq!(atoi_signed(&"defec8ed".to_string(), Base::new(16), false), Ok(0xDEFEC8ED));
		assert_eq!(atoi_signed(&"facefeed".to_string(), Base::new(16), false), Ok(0xFACEFEED));
		assert_eq!(atoi_signed(&"fee1dead".to_string(), Base::new(16), false), Ok(0xFEE1DEAD));
		assert_eq!(atoi_signed(&"feedcafe".to_string(), Base::new(16), false), Ok(0xFEEDCAFE));
		assert_eq!(atoi_signed(&"c0ffee".to_string(), Base::new(16), false), Ok(0xC0FFEE));
		assert_eq!(atoi_signed(&"e011cfd0".to_string(), Base::new(16), false), Ok(0xE011CFD0));
		assert_eq!(atoi_signed(&"face8d".to_string(), Base::new(16), false), Ok(0xFACE8D));
		assert_eq!(atoi_signed(&"feee".to_string(), Base::new(16), false), Ok(0xFEEE));
		assert_eq!(atoi_signed(&"cccccccc".to_string(), Base::new(16), false), Ok(0xCCCCCCCC));

		assert_eq!(atoi_unsigned(&"8badf00d".to_string(), Base::new(16), false), Ok(0x8BADF00D));
		assert_eq!(atoi_unsigned(&"abadbabe".to_string(), Base::new(16), false), Ok(0xABADBABE));
		assert_eq!(atoi_unsigned(&"1badb002".to_string(), Base::new(16), false), Ok(0x1BADB002));
		assert_eq!(atoi_unsigned(&"baadf00d".to_string(), Base::new(16), false), Ok(0xBAADF00D));
		assert_eq!(atoi_unsigned(&"badcab1e".to_string(), Base::new(16), false), Ok(0xBADCAB1E));
		assert_eq!(atoi_unsigned(&"baddcafe".to_string(), Base::new(16), false), Ok(0xBADDCAFE));
		assert_eq!(atoi_unsigned(&"beadface".to_string(), Base::new(16), false), Ok(0xBEADFACE));
		assert_eq!(atoi_unsigned(&"cafebabe".to_string(), Base::new(16), false), Ok(0xCAFEBABE));
		assert_eq!(atoi_unsigned(&"d15ea5e".to_string(), Base::new(16), false), Ok(0xD15EA5E));
		assert_eq!(atoi_unsigned(&"deadbabe".to_string(), Base::new(16), false), Ok(0xDEADBABE));
		assert_eq!(atoi_unsigned(&"deadbeef".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_unsigned(&"deaddead".to_string(), Base::new(16), false), Ok(0xDEADDEAD));
		assert_eq!(atoi_unsigned(&"deadfa11".to_string(), Base::new(16), false), Ok(0xDEADFA11));
		assert_eq!(atoi_unsigned(&"defec8ed".to_string(), Base::new(16), false), Ok(0xDEFEC8ED));
		assert_eq!(atoi_unsigned(&"facefeed".to_string(), Base::new(16), false), Ok(0xFACEFEED));
		assert_eq!(atoi_unsigned(&"fee1dead".to_string(), Base::new(16), false), Ok(0xFEE1DEAD));
		assert_eq!(atoi_unsigned(&"feedcafe".to_string(), Base::new(16), false), Ok(0xFEEDCAFE));
		assert_eq!(atoi_unsigned(&"c0ffee".to_string(), Base::new(16), false), Ok(0xC0FFEE));
		assert_eq!(atoi_unsigned(&"e011cfd0".to_string(), Base::new(16), false), Ok(0xE011CFD0));
		assert_eq!(atoi_unsigned(&"face8d".to_string(), Base::new(16), false), Ok(0xFACE8D));
		assert_eq!(atoi_unsigned(&"feee".to_string(), Base::new(16), false), Ok(0xFEEE));
		assert_eq!(atoi_unsigned(&"cccccccc".to_string(), Base::new(16), false), Ok(0xCCCCCCCC));

		assert_eq!(atoi_signed(&"8BADF00D".to_string(), Base::new(16), false), Ok(0x8BADF00D));
		assert_eq!(atoi_signed(&"ABADBABE".to_string(), Base::new(16), false), Ok(0xABADBABE));
		assert_eq!(atoi_signed(&"1BADB002".to_string(), Base::new(16), false), Ok(0x1BADB002));
		assert_eq!(atoi_signed(&"BAADF00D".to_string(), Base::new(16), false), Ok(0xBAADF00D));
		assert_eq!(atoi_signed(&"BADCAB1E".to_string(), Base::new(16), false), Ok(0xBADCAB1E));
		assert_eq!(atoi_signed(&"BADDCAFE".to_string(), Base::new(16), false), Ok(0xBADDCAFE));
		assert_eq!(atoi_signed(&"BEADFACE".to_string(), Base::new(16), false), Ok(0xBEADFACE));
		assert_eq!(atoi_signed(&"CAFEBABE".to_string(), Base::new(16), false), Ok(0xCAFEBABE));
		assert_eq!(atoi_signed(&"D15EA5E".to_string(), Base::new(16), false), Ok(0xD15EA5E));
		assert_eq!(atoi_signed(&"DEADBABE".to_string(), Base::new(16), false), Ok(0xDEADBABE));
		assert_eq!(atoi_signed(&"DEADBEEF".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_signed(&"DEADDEAD".to_string(), Base::new(16), false), Ok(0xDEADDEAD));
		assert_eq!(atoi_signed(&"DEADFA11".to_string(), Base::new(16), false), Ok(0xDEADFA11));
		assert_eq!(atoi_signed(&"DEFEC8ED".to_string(), Base::new(16), false), Ok(0xDEFEC8ED));
		assert_eq!(atoi_signed(&"FACEFEED".to_string(), Base::new(16), false), Ok(0xFACEFEED));
		assert_eq!(atoi_signed(&"FEE1DEAD".to_string(), Base::new(16), false), Ok(0xFEE1DEAD));
		assert_eq!(atoi_signed(&"FEEDCAFE".to_string(), Base::new(16), false), Ok(0xFEEDCAFE));
		assert_eq!(atoi_signed(&"C0FFEE".to_string(), Base::new(16), false), Ok(0xC0FFEE));
		assert_eq!(atoi_signed(&"E011CFD0".to_string(), Base::new(16), false), Ok(0xE011CFD0));
		assert_eq!(atoi_signed(&"FACE8D".to_string(), Base::new(16), false), Ok(0xFACE8D));
		assert_eq!(atoi_signed(&"FEEE".to_string(), Base::new(16), false), Ok(0xFEEE));
		assert_eq!(atoi_signed(&"CCCCCCCC".to_string(), Base::new(16), false), Ok(0xCCCCCCCC));

		assert_eq!(atoi_unsigned(&"8BADF00D".to_string(), Base::new(16), false), Ok(0x8BADF00D));
		assert_eq!(atoi_unsigned(&"ABADBABE".to_string(), Base::new(16), false), Ok(0xABADBABE));
		assert_eq!(atoi_unsigned(&"1BADB002".to_string(), Base::new(16), false), Ok(0x1BADB002));
		assert_eq!(atoi_unsigned(&"BAADF00D".to_string(), Base::new(16), false), Ok(0xBAADF00D));
		assert_eq!(atoi_unsigned(&"BADCAB1E".to_string(), Base::new(16), false), Ok(0xBADCAB1E));
		assert_eq!(atoi_unsigned(&"BADDCAFE".to_string(), Base::new(16), false), Ok(0xBADDCAFE));
		assert_eq!(atoi_unsigned(&"BEADFACE".to_string(), Base::new(16), false), Ok(0xBEADFACE));
		assert_eq!(atoi_unsigned(&"CAFEBABE".to_string(), Base::new(16), false), Ok(0xCAFEBABE));
		assert_eq!(atoi_unsigned(&"D15EA5E".to_string(), Base::new(16), false), Ok(0xD15EA5E));
		assert_eq!(atoi_unsigned(&"DEADBABE".to_string(), Base::new(16), false), Ok(0xDEADBABE));
		assert_eq!(atoi_unsigned(&"DEADBEEF".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_unsigned(&"DEADDEAD".to_string(), Base::new(16), false), Ok(0xDEADDEAD));
		assert_eq!(atoi_unsigned(&"DEADFA11".to_string(), Base::new(16), false), Ok(0xDEADFA11));
		assert_eq!(atoi_unsigned(&"DEFEC8ED".to_string(), Base::new(16), false), Ok(0xDEFEC8ED));
		assert_eq!(atoi_unsigned(&"FACEFEED".to_string(), Base::new(16), false), Ok(0xFACEFEED));
		assert_eq!(atoi_unsigned(&"FEE1DEAD".to_string(), Base::new(16), false), Ok(0xFEE1DEAD));
		assert_eq!(atoi_unsigned(&"FEEDCAFE".to_string(), Base::new(16), false), Ok(0xFEEDCAFE));
		assert_eq!(atoi_unsigned(&"C0FFEE".to_string(), Base::new(16), false), Ok(0xC0FFEE));
		assert_eq!(atoi_unsigned(&"E011CFD0".to_string(), Base::new(16), false), Ok(0xE011CFD0));
		assert_eq!(atoi_unsigned(&"FACE8D".to_string(), Base::new(16), false), Ok(0xFACE8D));
		assert_eq!(atoi_unsigned(&"FEEE".to_string(), Base::new(16), false), Ok(0xFEEE));
		assert_eq!(atoi_unsigned(&"CCCCCCCC".to_string(), Base::new(16), false), Ok(0xCCCCCCCC));
	}

	#[test]
	fn test_itoa_signed() {
		const TEST_VALUE_1: isize = 0xDEADBEEF;
		assert_eq!(itoa_signed(TEST_VALUE_1, Base::new(2), Case::Lower), "11011110101011011011111011101111");
		assert_eq!(itoa_signed(TEST_VALUE_1, Base::new(2), Case::Upper), "11011110101011011011111011101111");
		assert_eq!(itoa_signed(TEST_VALUE_1, Base::new(8), Case::Lower), "33653337357");
		assert_eq!(itoa_signed(TEST_VALUE_1, Base::new(8), Case::Upper), "33653337357");
		assert_eq!(itoa_signed(TEST_VALUE_1, Base::new(10), Case::Lower), "3735928559");
		assert_eq!(itoa_signed(TEST_VALUE_1, Base::new(10), Case::Upper), "3735928559");


		const TEST_VALUE_2: isize = -1;
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(2), Case::Lower), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(2), Case::Upper), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(8), Case::Lower), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(8), Case::Upper), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(10), Case::Lower), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(10), Case::Upper), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(16), Case::Lower), "-1");
		assert_eq!(itoa_signed(TEST_VALUE_2, Base::new(16), Case::Upper), "-1");

		const TEST_VALUE_3: isize = -2;
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(2), Case::Lower), "-10");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(2), Case::Upper), "-10");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(8), Case::Lower), "-2");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(8), Case::Upper), "-2");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(10), Case::Lower), "-2");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(10), Case::Upper), "-2");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(16), Case::Lower), "-2");
		assert_eq!(itoa_signed(TEST_VALUE_3, Base::new(16), Case::Upper), "-2");
	}

	#[test]
	fn test_itoa_unsigned() {
		const TEST_VALUE_1: usize = 0xDEADBEEF;
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(2), Case::Lower), "11011110101011011011111011101111");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(2), Case::Upper), "11011110101011011011111011101111");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(8), Case::Lower), "33653337357");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(8), Case::Upper), "33653337357");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(10), Case::Lower), "3735928559");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(10), Case::Upper), "3735928559");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(16), Case::Lower), "deadbeef");
		assert_eq!(itoa_unsigned(TEST_VALUE_1, Base::new(16), Case::Upper), "DEADBEEF");
	}

	#[test]
	fn test_atoi_signed() {
		assert_eq!(atoi_signed(&"11011110101011011011111011101111".to_string(), Base::new(2), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_signed(&"33653337357".to_string(), Base::new(8), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_signed(&"3735928559".to_string(), Base::new(10), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_signed(&"DEADBEEF".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_signed(&"deadbeef".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
	}

	#[test]
	fn test_atoi_unsigned() {
		assert_eq!(atoi_unsigned(&"11011110101011011011111011101111".to_string(), Base::new(2), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_unsigned(&"33653337357".to_string(), Base::new(8), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_unsigned(&"3735928559".to_string(), Base::new(10), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_unsigned(&"DEADBEEF".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
		assert_eq!(atoi_unsigned(&"deadbeef".to_string(), Base::new(16), false), Ok(0xDEADBEEF));
	}

	#[test]
	fn test_atoi_signed_check_error() {
		assert_eq!(atoi_signed(&"2".to_string(), Base::new(2), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"8".to_string(), Base::new(8), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"9".to_string(), Base::new(8), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"A".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"B".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"C".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"a".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"b".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"c".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"G".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"H".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"I".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-2".to_string(), Base::new(2), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-8".to_string(), Base::new(8), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-9".to_string(), Base::new(8), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-A".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-B".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-C".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-a".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-b".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-c".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-G".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-H".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_signed(&"-I".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
	}

	#[test]
	fn test_atoi_unsigned_check_error() {
		assert_eq!(atoi_unsigned(&"2".to_string(), Base::new(2), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"8".to_string(), Base::new(8), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"9".to_string(), Base::new(8), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"A".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"B".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"C".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"a".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"b".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"c".to_string(), Base::new(10), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"G".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"H".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
		assert_eq!(atoi_unsigned(&"I".to_string(), Base::new(16), false), Err(ParseIntKind::InvalidCharacter));
	}

	#[test]
	fn test_atoi_signed_overflow() {
		assert_eq!(atoi_signed(&"-1".to_string(), Base::new(2), false), Ok(-1));
		assert_eq!(atoi_signed(&"111111111111111111111111111111111111111111111111111111111111111".to_string(), Base::new(2), false), Ok(9223372036854775807));
		assert_eq!(atoi_signed(&"11111111111111111111111111111111".to_string(), Base::new(2), false), Ok(4294967295));
		assert_eq!(atoi_signed(&"1111111111111111".to_string(), Base::new(2), false), Ok(65535));
		assert_eq!(atoi_signed(&"11111111".to_string(), Base::new(2), false), Ok(255));
		assert_eq!(atoi_signed(&"1111".to_string(), Base::new(2), false), Ok(15));

		assert_eq!(atoi_signed(&"7FFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Ok(0x7FFFFFFFFFFFFFFF));
		assert_eq!(atoi_signed(&"FFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_signed(&"7FFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_signed(&"FFFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_signed(&"7FFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_signed(&"FFFFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_signed(&"7FFFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));

	}
	
	#[test]
	fn test_atoi_unsigned_overflow() {
		assert_eq!(atoi_unsigned(&"1111111111111111111111111111111111111111111111111111111111111111".to_string(), Base::new(2), false), Ok(18446744073709551615));
		assert_eq!(atoi_unsigned(&"111111111111111111111111111111111111111111111111111111111111111".to_string(), Base::new(2), false), Ok(9223372036854775807));
		assert_eq!(atoi_unsigned(&"11111111111111111111111111111111".to_string(), Base::new(2), false), Ok(4294967295));
		assert_eq!(atoi_unsigned(&"1111111111111111".to_string(), Base::new(2), false), Ok(65535));
		assert_eq!(atoi_unsigned(&"11111111".to_string(), Base::new(2), false), Ok(255));
		assert_eq!(atoi_unsigned(&"1111".to_string(), Base::new(2), false), Ok(15));

		assert_eq!(atoi_unsigned(&"7FFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Ok(0x7FFFFFFFFFFFFFFF));
		assert_eq!(atoi_unsigned(&"FFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Ok(0xFFFFFFFFFFFFFFFF));
		assert_eq!(atoi_unsigned(&"FFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_unsigned(&"7FFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_unsigned(&"FFFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_unsigned(&"7FFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_unsigned(&"FFFFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
		assert_eq!(atoi_unsigned(&"7FFFFFFFFFFFFFFFFFF".to_string(), Base::new(16), false), Err(ParseIntKind::Overflow));
	}

	#[test]
	fn test_atoi_signed_empty() {
		assert_eq!(atoi_signed(&"".to_string(), Base::new(2), false), Err(ParseIntKind::Empty));
		assert_eq!(atoi_signed(&"".to_string(), Base::new(8), false), Err(ParseIntKind::Empty));
		assert_eq!(atoi_signed(&"".to_string(), Base::new(10), false), Err(ParseIntKind::Empty));
		assert_eq!(atoi_signed(&"".to_string(), Base::new(16), false), Err(ParseIntKind::Empty));
		assert_eq!(atoi_signed(&"".to_string(), Base::new(32), false), Err(ParseIntKind::Empty));
		assert_eq!(atoi_signed(&"".to_string(), Base::new(36), false), Err(ParseIntKind::Empty));
	}

	#[test]
	fn test_itoa_high_base() {
		assert_eq!(itoa_signed(916381, Base::new(32), Case::Lower), "rust");
		assert_eq!(itoa_signed(916381, Base::new(32), Case::Upper), "RUST");
		assert_eq!(itoa_signed(1299629, Base::new(36), Case::Lower), "rust");
		assert_eq!(itoa_signed(1299629, Base::new(36), Case::Upper), "RUST");
		assert_eq!(itoa_signed(19196013312925, Base::new(32), Case::Lower), "hellorust");
		assert_eq!(itoa_signed(19196013312925, Base::new(32), Case::Upper), "HELLORUST");
		assert_eq!(itoa_signed(49102990553261, Base::new(36), Case::Lower), "hellorust");
		assert_eq!(itoa_signed(49102990553261, Base::new(36), Case::Upper), "HELLORUST");
		assert_eq!(itoa_signed(1767707668033969, Base::new(36), Case::Lower), "helloworld");
		assert_eq!(itoa_signed(1767707668033969, Base::new(36), Case::Upper), "HELLOWORLD");

		assert_eq!(itoa_unsigned(916381, Base::new(32), Case::Lower), "rust");
		assert_eq!(itoa_unsigned(916381, Base::new(32), Case::Upper), "RUST");
		assert_eq!(itoa_unsigned(1299629, Base::new(36), Case::Lower), "rust");
		assert_eq!(itoa_unsigned(1299629, Base::new(36), Case::Upper), "RUST");
		assert_eq!(itoa_unsigned(19196013312925, Base::new(32), Case::Lower), "hellorust");
		assert_eq!(itoa_unsigned(19196013312925, Base::new(32), Case::Upper), "HELLORUST");
		assert_eq!(itoa_unsigned(49102990553261, Base::new(36), Case::Lower), "hellorust");
		assert_eq!(itoa_unsigned(49102990553261, Base::new(36), Case::Upper), "HELLORUST");
		assert_eq!(itoa_unsigned(1767707668033969, Base::new(36), Case::Lower), "helloworld");
		assert_eq!(itoa_unsigned(1767707668033969, Base::new(36), Case::Upper), "HELLOWORLD");

	}

	#[test]
	fn test_atoi_high_base() {
		assert_eq!(atoi_signed(&"rust".to_string(), Base::new(32), false), Ok(916381));
		assert_eq!(atoi_signed(&"RUST".to_string(), Base::new(32), false), Ok(916381));
		assert_eq!(atoi_signed(&"rust".to_string(), Base::new(36), false), Ok(1299629));
		assert_eq!(atoi_signed(&"RUST".to_string(), Base::new(36), false), Ok(1299629));
		assert_eq!(atoi_signed(&"hellorust".to_string(), Base::new(32), false), Ok(19196013312925));
		assert_eq!(atoi_signed(&"HELLORUST".to_string(), Base::new(32), false), Ok(19196013312925));
		assert_eq!(atoi_signed(&"hellorust".to_string(), Base::new(36), false), Ok(49102990553261));
		assert_eq!(atoi_signed(&"HELLORUST".to_string(), Base::new(36), false), Ok(49102990553261));
		assert_eq!(atoi_signed(&"helloworld".to_string(), Base::new(36), false), Ok(1767707668033969));
		assert_eq!(atoi_signed(&"HELLOWORLD".to_string(), Base::new(36), false), Ok(1767707668033969));

		assert_eq!(atoi_unsigned(&"rust".to_string(), Base::new(32), false), Ok(916381));
		assert_eq!(atoi_unsigned(&"RUST".to_string(), Base::new(32), false), Ok(916381));
		assert_eq!(atoi_unsigned(&"rust".to_string(), Base::new(36), false), Ok(1299629));
		assert_eq!(atoi_unsigned(&"RUST".to_string(), Base::new(36), false), Ok(1299629));
		assert_eq!(atoi_unsigned(&"hellorust".to_string(), Base::new(32), false), Ok(19196013312925));
		assert_eq!(atoi_unsigned(&"HELLORUST".to_string(), Base::new(32), false), Ok(19196013312925));
		assert_eq!(atoi_unsigned(&"hellorust".to_string(), Base::new(36), false), Ok(49102990553261));
		assert_eq!(atoi_unsigned(&"HELLORUST".to_string(), Base::new(36), false), Ok(49102990553261));
		assert_eq!(atoi_unsigned(&"helloworld".to_string(), Base::new(36), false), Ok(1767707668033969));
		assert_eq!(atoi_unsigned(&"HELLOWORLD".to_string(), Base::new(36), false), Ok(1767707668033969));
	}

	#[test]
	fn test_c_atoi() {
		assert_eq!(atoi!(&"0".to_string()), 0);
		assert_eq!(atoi!(&"1".to_string()), 1);
		assert_eq!(atoi!(&"2".to_string()), 2);
		assert_eq!(atoi!(&"3".to_string()), 3);
		assert_eq!(atoi!(&"4".to_string()), 4);
		assert_eq!(atoi!(&"5".to_string()), 5);
	}

	#[test]
	fn test_formatting() {
		assert_eq!(format_text(&"Hello".to_string(), FormatOptions {
			padding: FormatDirection::Left(10),
			filling: Some('_'),
		}), "Hello_____");
		assert_eq!(format_text(&"Hello".to_string(), FormatOptions {
			padding: FormatDirection::Right(10),
			filling: Some('_'),
		}), "_____Hello");

		assert_eq!(format_text(&"Hello world".to_string(), FormatOptions {
			padding: FormatDirection::Left(10),
			filling: Some('_'),
		}), "Hello world");
		
		assert_eq!(format_text(&"Hello world".to_string(), FormatOptions {
			padding: FormatDirection::Right(10),
			filling: Some('_'),
		}), "Hello world");

		assert_eq!(format_text(&"Hello world".to_string(), FormatOptions {
			padding: FormatDirection::Left(20),
			filling: Some('_'),
		}), "Hello world_________");
		
		assert_eq!(format_text(&"Hello world".to_string(), FormatOptions {
			padding: FormatDirection::Right(20),
			filling: Some('_'),
		}), "_________Hello world");
		
		assert_eq!(format_text(&"Hello world".to_string(), FormatOptions {
			padding: FormatDirection::Right(15),
			filling: Some('_'),
		}), "____Hello world");
		
		assert_eq!(format_text(&"Hello world".to_string(), FormatOptions {
			padding: FormatDirection::Center(15),
			filling: Some('_'),
		}), "Hello world"); /* should not change */
	}

	#[test]
	fn test_c_format() {
		assert_eq!(c_format("Hello, %s!".to_string(),
			vec![
				CFormatArgument::String("world".to_string())
			]
		), "Hello, world!");
		assert_eq!(c_format("My favorite number is %d.".to_string(),
			vec![
				CFormatArgument::Int32(42)
			]
		), "My favorite number is 42.");
		assert_eq!(c_format("%-30s".to_string(),
			vec![
				CFormatArgument::String("Left aligned".to_string())
			]
		), "Left aligned                  ");
		assert_eq!(c_format("%30s".to_string(),
			vec![
				CFormatArgument::String("Right aligned".to_string())
			]
		), "                 Right aligned");
		assert_eq!(c_format("%*s".to_string(),
			vec![
				CFormatArgument::IntSize(-30),
				CFormatArgument::String("Left aligned".to_string())
			]
		), "Left aligned                  ");
		assert_eq!(c_format("int: %d; hex: %x %X; oct: %o".to_string(),
			vec![
				CFormatArgument::Int32(42),
				CFormatArgument::UInt32(42),
				CFormatArgument::UInt32(42),
				CFormatArgument::UInt32(42),
			]
		), "int: 42; hex: 2a 2A; oct: 52");
		assert_eq!(c_format("%c, %c, %c".to_string(),
			vec![
				CFormatArgument::Character('a'),
				CFormatArgument::Character('b'),
				CFormatArgument::Character('c'),
			]
		), "a, b, c");
		assert_eq!(c_format("*%d*".to_string(),
			vec![
				CFormatArgument::Int32(959),
			]
		), "*959*");
		assert_eq!(c_format("*%2d*".to_string(),
			vec![
				CFormatArgument::Int32(959),
			]
		), "*959*");
		assert_eq!(c_format("*%10d*".to_string(),
			vec![
				CFormatArgument::Int32(959),
			]
		), "*       959*");
		assert_eq!(c_format("*%-10d*".to_string(),
			vec![
				CFormatArgument::Int32(959),
			]
		), "*959       *");
		
		/*                   i8 %i8 =i8 */
		assert_eq!(c_format("%d%%%d%c%d".to_string(),
			vec![
				CFormatArgument::Int32(14),
				CFormatArgument::Int32(10),
				CFormatArgument::Character('='),
				CFormatArgument::Int32(4),
			]
		), "14%10=4");
	}
}
