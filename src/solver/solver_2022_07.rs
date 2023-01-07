use super::utils;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[grammar = "solver/solver_2022_07.pest"]
struct SantaParser;

struct Folder {
    path: String,
    file_size: u32,
    sub_folders: HashSet<String>,
}

impl Folder {
    fn generate_path(path: &Vec<String>) -> String {
        format!("/{}", path.join("/"))
    }

    fn get_total_size(&self, size_map: &HashMap<String, u32>) -> Option<u32> {
        match size_map.get(&self.path) {
            Some(size) => Some(*size),
            None => {
                let sub_folder_size: Option<u32> =
                    self.sub_folders.iter().fold(Some(0), |acc, subfolder| {
                        Some(acc? + size_map.get(subfolder)?)
                    });
                Some(sub_folder_size? + self.file_size)
            }
        }
    }
}

impl From<String> for Folder {
    fn from(path: String) -> Self {
        Self {
            path,
            file_size: 0,
            sub_folders: HashSet::new(),
        }
    }
}

impl From<&Vec<String>> for Folder {
    fn from(path: &Vec<String>) -> Self {
        Folder::from(Folder::generate_path(path))
    }
}

fn get_size_map(input: &str) -> HashMap<String, u32> {
    let pairs = SantaParser::parse(Rule::Program, input).unwrap_or_else(|e| panic!("{}", e));
    let program = pairs.peek().unwrap();
    let mut current_directory = Vec::<String>::new();
    let mut folder_map = HashMap::<String, Folder>::new();
    for statement in program.into_inner() {
        match statement.as_rule() {
            Rule::CDCommand => {
                let path = statement.into_inner().peek().unwrap();
                match path.as_rule() {
                    Rule::RootFolder => {
                        current_directory = vec![];
                    }
                    Rule::ParentFolder => {
                        current_directory.pop();
                    }
                    Rule::FolderName => {
                        current_directory.push(path.as_str().to_string());
                    }
                    other => panic!("syntax error: cd command cannot have {:?}", other),
                }
                let folder = Folder::from(&current_directory);
                folder_map.entry(folder.path.clone()).or_insert(folder);
            }
            Rule::LSCommand => {
                let pairs: Vec<Pair<Rule>> = statement.into_inner().collect();
                let mut sub_folders = HashSet::<String>::new();
                let mut file_size = 0;
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::DirectoryInfo => {
                            current_directory
                                .push(pair.into_inner().peek().unwrap().as_str().to_string());
                            sub_folders.insert(Folder::generate_path(&current_directory));
                            current_directory.pop();
                        }
                        Rule::FileInfo => {
                            file_size += pair
                                .into_inner()
                                .peek()
                                .unwrap()
                                .as_str()
                                .parse::<u32>()
                                .unwrap();
                        }
                        other => panic!("syntax error: ls command cannot have {:?}", other),
                    }
                }
                let path = Folder::generate_path(&current_directory);
                folder_map.entry(path).and_modify(|f| {
                    f.sub_folders = sub_folders;
                    f.file_size = file_size;
                });
            }
            other => panic!("syntax error: statement cannot be {:?}", other),
        }
    }
    let mut size_map: HashMap<String, u32> = HashMap::new();
    while size_map.len() < folder_map.len() {
        for folder in folder_map.values() {
            if size_map.contains_key(&folder.path) {
                continue;
            }
            match folder.get_total_size(&size_map) {
                Some(value) => {
                    size_map.insert(folder.path.clone(), value);
                }
                None => {}
            }
        }
    }
    size_map
}

fn solve_first_part(input: &str) -> u32 {
    let size_map = get_size_map(input);
    size_map
        .values()
        .fold(0, |sum, size| if *size < 100000 { sum + size } else { sum })
}

fn solve_second_part(input: &str) -> u32 {
    let size_map = get_size_map(input);
    let available_size = 70000000;
    let required_size = 30000000;
    let used_space = size_map.get("/").unwrap();
    let free_space = available_size - used_space;
    let needed_space = required_size - free_space;

    size_map.values().fold(*used_space, |candidate, size| {
        if *size > needed_space && *size < candidate {
            *size
        } else {
            candidate
        }
    })
}

pub fn solve() -> (u32, u32) {
    let input = utils::get_input("inputs/2022_07.txt");
    (solve_first_part(&input[..]), solve_second_part(&input[..]))
}

#[cfg(test)]
mod tests {
    static EXAMPLE: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
    #[test]
    fn should_solve_first_part_example() {
        assert_eq!(super::solve_first_part(EXAMPLE), 95437);
    }

    #[test]
    fn should_solve_second_part_example() {
        assert_eq!(super::solve_second_part(EXAMPLE), 24933642);
    }
}
