fn main() {
    let word = String::from("one two three four five six seven eight nine ten");

    println!("{}", nth_word(&word, 11));
}

fn nth_word(s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();
    let mut word_count = 1;
    let mut first_letter = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            if word_count == n {
                return &s[first_letter..i];
            }

            word_count += 1;
            first_letter = i + 1;
        }
    }

    if word_count == n {
        return &s[first_letter..];
    }

    ""
}