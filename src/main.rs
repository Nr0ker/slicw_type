// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5

//     s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5 here, but s no longer has any content that we
//     // could meaningfully use with the value 5, so word is now totally invalid!
// }


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// let s = String::from("hello"); 

// let slice = &s[0..2]; // the slice is like a borrower & but we can specify what bytes of the String we want to borrow
// where 0 - the fist symbol is the starting byte 
// and .. till - 2 is the ending byte
// let slice = &s[..2]; // if we want to start from zero we can also just type .. and the end byte for example ..2
// which will go from byte 0 to byte 2\

// let len = s.len();
// let slice = &s[3..len]; // we can also do this, like take the byte 3 and len of the s and go till the end of the s as the len of s is 5
// let slice = &s[3..]; // or just as with the start of the bytes we can do the 3.. and add nothing more, so the slice will go till the end of the string
// let slice = &s[..]; // or just why bother placing numbers at the first place, when we can have the whole String

//  Note: String slice range indices must occur at valid UTF-8 character boundaries.
//  If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.


fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}