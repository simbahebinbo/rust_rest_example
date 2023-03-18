fn main() {
    let mut string1: String = "String1".to_string();
    let string2: String = String::from("String2");

    let string_literal: &str = "String_literal";

    let string_slice: &str = &string1[..];

    let string3 = String::from(string_slice);

    //string1: String1, string2: String2, string_literal: String_literal, string_slice: String1, string3: String1
    println!("string1: {}, string2: {}, string_literal: {}, string_slice: {}, string3: {}",
             string1, string2, string_literal, string_slice, string3);

    string1.push_str(" and new part");
    let string_slice2 = &string1[..];
    //string1: String1 and new part, string_slice2: String1 and new part
    println!("string1: {}, string_slice2: {}", string1, string_slice2);
}

