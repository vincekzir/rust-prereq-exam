// Function: pattern_1
// Description: Returns true if the last two strings in the vector start with `BLOKC`.
// Parameters:
// - input: A vector of strings.
// Returns: True if the last two strings in the vector start with `BLOKC`, false otherwise.
pub fn pattern_1(input: Vec<String>) -> bool {
    // Check if input has at least 2 elements
    if input.len() < 2 {
        return false 
    }
     // Check if the second-to-last element ends with "BLOKC" and the last element starts with "BLOKC"
    return &input[input.len() - 2][..5] == "BLOKC" && input[input.len() - 1].starts_with("BLOKC");
}

// Function: pattern_2
// Description: Returns true if the first and last string in the vector start with `BLOKC`.
// Parameters:
// - input: A vector of strings.
// Returns: True if the first and last string in the vector start with `BLOKC`, false otherwise.
pub fn pattern_2(input: Vec<String>) -> bool {
     if input.len() < 2 {
        return false;
    }
    // Check if the first element starts with "BLOKC" and the last element starts with "BLOKC"
    return &input[0][..5] == "BLOKC" && input[input.len() - 1].starts_with("BLOKC"); // Two ways to do splicing
}

// Function: pattern_3
// Description: Returns true if a string contains all the letters of the word 'BLOKC'.
// Parameters:
// - input: A string.
// Returns: True if a string contains all the letters of the word 'BLOKC', false otherwise.
pub fn pattern_3(input: &str) -> bool {
    // Iterate over each character in the string "BLOKC"
    for letter in "BLOKC".chars() {
        // Check if the input string contains the current letter
        if !input.contains(letter) {
            return false;  // Return false if the input string does not contain the current letter
        }
    }
    true // Return true if all letters in "BLOKC" are found
}

// Function: pattern_4
// Description: Returns a string that rearranges its characters in alphabetical order.
// Parameters:
// - input: A string.
// Returns: A string that rearranges its characters in alphabetical order.
pub fn pattern_4(input: &str) -> String {
    let mut char_vec: Vec<char> = input.chars().collect();   // Convert the input string into a vector of characters
    char_vec.sort(); // Sort the characters in ascending order
    char_vec.into_iter().collect() // Convert the sorted characters back into a string and return

    // Another way to do it??
    // for i in 0..char_vec.len() {
    //     for j in (i + 1)..char_vec.len() {
    //         if char_vec[i] > char_vec[j] {
    //             char_vec.swap(i, j);
    //         }
    //     }
    // }
    // char_vec.into_iter().collect()
}

// Function: pattern_5
// Description: Returns a string where characters similar to the first character are converted.
// Parameters:
// - input: A string.
// Returns: A string with characters similar to the first character converted.
pub fn pattern_5(input: &str) -> String {
    let mut char_vec: Vec<char> = input.chars().collect();  // Convert the input string into a vector of characters
    
    // Check if the vector is not empty, and get the first character
    if let Some(&f_char) = char_vec.first() {
        // Replace all characters in the vector except the first one with the first character
        for i in 1..char_vec.len() {
            char_vec[i] = f_char; 
            }
        }
    
    // Convert the 'new' vector back into a string and return it
    char_vec.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_pattern_1() {
		let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOCKCHANG".to_string(),
		];
		assert!(pattern_1(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(!pattern_1(strs_2));
	}

    // For Another Testing
    // #[test]
	// fn test_pattern_1() {
	// 	let strs_1 = vec![
	// 		"HELLO".to_string(),
	// 		"RUST".to_string(),
	// 		"BLOKC".to_string(),
	// 		"BLOKCCHANG".to_string(),
	// 	];
	// 	assert!(pattern_1(strs_1));

    //     let strs_2 = vec![
	// 		"BLOKCS".to_string(),
	// 		"WORLD".to_string(),
	// 		"BLOKC".to_string(),
	// 	];
	// 	assert!(!pattern_1(strs_2));
	// }

    #[test]
    fn test_pattern_2() {
        let strs_1 = vec![
			"HELLO".to_string(),
			"RUST".to_string(),
			"BLOKC".to_string(),
			"BLOCKCHANG".to_string(),
		];
		assert!(!pattern_2(strs_1));

        let strs_2 = vec![
			"BLOKCS".to_string(),
			"WORLD".to_string(),
			"BLOKC".to_string(),
		];
		assert!(pattern_2(strs_2));
    }

    #[test]
    fn test_pattern_3() {
		assert!(pattern_3("BLOKCBUSTER"));
		assert!(pattern_3("THEBLOKC"));
        assert!(pattern_3("KCOLB"));
        assert!(pattern_3("B*L*O*C*K"));
        assert!(!pattern_3("BLOB"));
        assert!(!pattern_3("IDONTKNOW"));
    }

    #[test]
    fn test_pattern_4() {
        assert_eq!(pattern_4("BLOKC"), "BCKLO");
        assert_eq!(pattern_4("HELLO"), "EHLLO");
        assert_eq!(pattern_4("EDCBA"), "ABCDE");
    }

    #[test]
    fn test_pattern_5() {
        assert_eq!(pattern_5("BLOCK"), "BBBBB");
        assert_eq!(pattern_5("HELLO"), "HHHHH");
        assert_eq!(pattern_5("RUST"), "RRRR");
        assert_eq!(pattern_5("CONGRATS"), "CCCCCCCC");
    }
}
