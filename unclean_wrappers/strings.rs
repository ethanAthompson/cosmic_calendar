use leptos::{
    leptos_dom::logging::console_log, RwSignal, SignalGet, SignalGetUntracked, SignalUpdate,
};

/// Returns the initials of the entered string
///
/// input => represents your word
/// debug => does it actually get the characters?
///
/// ```markdown
///
/// # "Eggs are Cool" -> "EaC"
///
///
/// ```
///
pub fn get_initials(input: String, debug: bool) -> String {
    let initial: String = input
        .split_whitespace()
        .filter_map(|word| {
            if debug {
                console_log(&format!(
                    "get_initials/.filter_map-word {:?}",
                    word.chars().as_str()
                ));
            }

            word.chars().next()
        })
        .collect();

    return initial;
}

/// Modifies Left signal to contain the input
/// ===
/// left => Signal that includes filtered item from input
/// right => Vector that will be matched
/// input => the word that will be checked in the right vector
///
/// # Example
/// ```rust
///
/// let left = create_rw_signal(vec!["Apples", "Oranges", "Blueberries"]);
/// let right = create_rw_signal(Vec::new());
/// let input = create_rw_signal("Oranges".to_string());
///
/// filtered_vec(&left, &right, &input);
///
/// # These are the same, because the provided method returns your input in a Vec
/// assert_eq!(&left.get(), vec!["Oranges"]);
///
/// ```
///
pub fn filtered_vec(
    left: &RwSignal<Vec<String>>,
    right: &RwSignal<Vec<String>>,
    input: &RwSignal<String>,
    debug: bool,
) {
    left.update(|item| {
        let matched = matching_left(&left, input.get(), debug, true);

        if debug {
            console_log(&format!(
                "filtered_vec/left.update-item/matched: {:?}",
                matched
            ));
        }

        *item = matched;
    });
}

/// Returns a vector containing the matching input
/// used in [`filtered_vec()`]
/// ===
/// left => the vector to find the specific item in
/// right => the word you that should be in the vector
/// debug => does it actually match?
/// hidden => do you want to create a dependency or not?
///
/// # Example
/// ```rust
///
/// let left = create_rw_signal(vec!["Apples", "Oranges", "Blueberries"]);
/// let input = create_rw_signal("Oranges".to_string());
///  
/// let item_found = matching_left(&left, input);
///
/// assert_eq!(item_found, vec!["Oranges"]);
///
/// ```
pub fn matching_left(
    left: &RwSignal<Vec<String>>,
    input: String,
    debug: bool,
    hidden: bool,
) -> Vec<String> {
    let mut matched: Vec<String>;

    if hidden {
        matched = left
            .get()
            .into_iter()
            .filter(|item| item.contains(&input).to_owned())
            .collect();
    } else {
        matched = left
            .get_untracked()
            .into_iter()
            .filter(|item| item.contains(&input).to_owned())
            .collect();
    }

    if debug {
        console_log(&format!("matching_left/matched: {:?}", matched));
    }

    return matched;
}
