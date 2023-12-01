use std::{fs, path::Path};

pub fn for_every_line(path: &str, mut handler: impl FnMut(&str)) {
    let file_content = fs::read_to_string(Path::new(path)).expect(&format!("failed to read from file {}", path));

    for line in file_content.split("\n") {
        let text: &str = line.trim();

        if text.is_empty() {
            continue;
        }

        handler(text);
    }
}
