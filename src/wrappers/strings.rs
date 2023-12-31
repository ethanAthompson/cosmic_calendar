use leptos::{
    leptos_dom::logging::console_log, RwSignal, SignalGet, SignalGetUntracked, SignalUpdate,
};

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

/// Left => Signal that includes Filtered Items from input
/// Right => Vector that will be matched
///
pub fn filtered_vec(
    left: &RwSignal<Vec<String>>,
    right: &RwSignal<Vec<String>>,
    input: &RwSignal<String>,
) {
    left.update(|item| {
        let matched: Vec<String> = right
            .get_untracked()
            .into_iter()
            .filter(|list| list.contains(input.get_untracked().clone().as_str()).to_owned())
            .collect();

        *item = matched;
    });
}


/// Left => The vector that you want to find a certain item
/// input => the str that you want to find inside the vector
///
pub fn matching_left(left: &RwSignal<Vec<String>>, input: String) -> Vec<String>{
    let matched: Vec<String> = left
        .get()
        .into_iter()
        .filter(|item| item.contains(&input).to_owned())
        .collect();

    return matched;
}
