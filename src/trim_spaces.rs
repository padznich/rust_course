
pub fn test_trim_spaces() {

	// Print description

	let row_1 = "  abc1";
	let row_2 = "abc2   ";
	let row_3 = "  abc3      ";
	let row_4 = " abc 4 ";

	println!("{:?} \tfor {:?}", trim_spaces(row_1), row_1);
	println!("{:?} \tfor {:?}", trim_spaces(row_2), row_2);
	println!("{:?} \tfor {:?}", trim_spaces(row_3), row_3);
	println!("{:?} \tfor {:?}", trim_spaces(row_4), row_4);

	// Test itself

	assert_eq!(trim_spaces(row_1), "abc1");
	assert_eq!(trim_spaces(row_2), "abc2");
	assert_eq!(trim_spaces(row_3), "abc3");
	assert_eq!(trim_spaces(row_4), "abc 4");
}

pub fn trim_spaces(row: &str) -> &str {
	/*

	*/

	// check the beginning of the row
	let mut start = 0;
	for (index, item) in row.chars().enumerate() {
		if item != ' ' {
			start = index;
			break;
		}
	}

	// check the end of the row
	let mut end = 0;
	for (index, item) in row.chars().rev().enumerate() {
		if item != ' ' {
			end = row.len() - index;
			break;
		}
	}

	&row[start..end]

}