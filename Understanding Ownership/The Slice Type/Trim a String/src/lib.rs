pub fn trimmed_space(s: &str) -> &str {
    let chars = s.chars();

    let mut first_non_space = s.len();
    let mut last_non_space = 0;

    for (i, c) in chars.enumerate() {
        if c != ' ' {
            if first_non_space == s.len() {
                first_non_space = i
            }
            last_non_space = i
        }
    }

    if last_non_space == 0 {
        &s[first_non_space..]
    } else {
        &s[first_non_space..=last_non_space]
    }
}
