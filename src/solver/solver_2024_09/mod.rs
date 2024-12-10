mod input;

use super::Solver;
use input::INPUT;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Space {
    Empty,
    File(usize),
}

#[derive(Clone)]
struct File {
    id: usize,
    size: usize,
    start: usize,
}

fn parse_input(input: &str) -> Vec<Space> {
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            let space = if i % 2 == 0 {
                Space::File(i / 2)
            } else {
                Space::Empty
            };
            vec![space; num as usize]
        })
        .collect()
}

fn get_files_and_empty_spaces(input: &str) -> (Vec<File>, HashMap<usize, Vec<usize>>) {
    let mut files = Vec::new();
    let mut empty_spaces = HashMap::new();
    let mut current_position = 0usize;
    for (i, c) in input.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(File {
                id: i / 2,
                size,
                start: current_position,
            });
        } else {
            empty_spaces
                .entry(size)
                .or_insert(Vec::new())
                .push(current_position);
        };
        current_position += size;
    }
    (files, empty_spaces)
}

#[derive(Clone)]
pub struct Solver202409 {
    disk: Vec<Space>,
    empty_spaces: HashMap<usize, Vec<usize>>,
    files: Vec<File>,
}

impl From<&str> for Solver202409 {
    fn from(value: &str) -> Self {
        let (files, empty_spaces) = get_files_and_empty_spaces(value);
        Self {
            disk: parse_input(value),
            files,
            empty_spaces,
        }
    }
}
impl Default for Solver202409 {
    fn default() -> Self {
        INPUT.into()
    }
}

impl Solver202409 {
    fn remove_empty_spaces_at_end(&mut self) {
        while self.disk.last().unwrap() == &Space::Empty {
            self.disk.pop();
        }
    }
    fn get_first_empty_space_index_in_disk(&self, skip: usize) -> Option<usize> {
        self.disk
            .iter()
            .skip(skip)
            .position(|space| *space == Space::Empty)
            .map(|pos| pos + skip)
    }

    fn defragmentation(&mut self) {
        self.remove_empty_spaces_at_end();

        let mut empty_space_index = self.get_first_empty_space_index_in_disk(0);
        while let Some(i) = empty_space_index {
            self.disk[i] = self.disk.pop().unwrap();
            self.remove_empty_spaces_at_end();
            empty_space_index = self.get_first_empty_space_index_in_disk(i + 1);
        }
    }

    fn get_first_empty_space_index_for_file(&self, file: &File) -> Option<(usize, usize)> {
        (file.size..10).fold(None, |acc, size| {
            let location = self.empty_spaces.get(&size).and_then(|v| v.get(0));
            if location.is_some_and(|&location| {
                location < file.start && acc.is_none_or(|(_, acc_location)| location < acc_location)
            }) {
                Some((size, *location.unwrap()))
            } else {
                acc
            }
        })
    }

    fn insert_empty_spaces(&mut self, size: usize, location: usize) {
        let vector = self.empty_spaces.entry(size).or_insert(Vec::new());
        match vector.binary_search(&location) {
            Ok(_) => {
                panic!("Location already exists")
            }
            Err(index) => vector.insert(index, location),
        }
    }

    fn defragmentation_with_moving_whole_files(&mut self) {
        let mut files = self.files.clone();
        for file in files.iter_mut().rev() {
            if let Some((size, location)) = self.get_first_empty_space_index_for_file(file) {
                file.start = location;
                self.empty_spaces.entry(size).and_modify(|v| {
                    v.remove(0);
                });
                if size > file.size {
                    self.insert_empty_spaces(size - file.size, location + file.size);
                }
            }
        }
        self.files = files;
    }

    fn checksum_from_files(&self) -> usize {
        self.files
            .iter()
            .flat_map(|file| (file.start..file.start + file.size).map(|i| (i, file.id)))
            .fold(0, |acc, (id, location)| acc + (id * location))
    }

    fn checksum_from_disk(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .fold(0, |acc, (i, space)| match space {
                Space::Empty => acc,
                Space::File(file) => acc + (i * file),
            })
    }
}

impl Solver<usize, usize> for Solver202409 {
    fn solve_first_part(&self) -> usize {
        let mut cloned_self = self.clone();
        cloned_self.defragmentation();
        cloned_self.checksum_from_disk()
    }
    fn solve_second_part(&self) -> usize {
        let mut cloned_self = self.clone();
        cloned_self.defragmentation_with_moving_whole_files();
        cloned_self.checksum_from_files()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "2333133121414131402";
    #[test]
    fn solve_first_part() {
        assert_eq!(Solver202409::from(EXAMPLE).solve_first_part(), 1928);
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(Solver202409::from(EXAMPLE).solve_second_part(), 2858);
    }
}
