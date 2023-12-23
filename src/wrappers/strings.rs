use leptos::leptos_dom::logging::console_log;

pub fn get_initials(input: String) -> String {
    let initial: String = input
        // removes whitespace  -> "E g g" = "Egg"
        .split_whitespace()
        // filters out the 1st letter in each word,
        // parameter: *word* represents your input
        // chars() is used to split the String into each word
        // next() is used to jump to the next word
        .filter_map(|word| {
            // console_log(word.chars().as_str());

            word.chars().next()
        })
        // combines the vec to a string => ["E", "A", "C"] => "EAC"
        .collect();

    // so now you have "EAC" or the initials you want
    return initial;
}
