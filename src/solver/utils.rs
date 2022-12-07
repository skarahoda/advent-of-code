use std::fs;

pub(in super) fn get_input(path: &str) -> String {
    let mut content = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    if content.chars().last().unwrap() == "\n".chars().next().unwrap() {
        content.pop();
    }
    content
}
