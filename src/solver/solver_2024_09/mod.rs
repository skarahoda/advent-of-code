mod input;

use super::Solver;
use input::INPUT;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Space {
    Empty,
    File(usize),
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

#[derive(Clone)]
pub struct Solver202409 {
    disk: Vec<Space>,
}

impl From<&str> for Solver202409 {
    fn from(value: &str) -> Self {
        Self {
            disk: parse_input(value),
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
    fn get_first_empty_space_index(&self, skip: usize) -> Option<usize> {
        self.disk
            .iter()
            .skip(skip)
            .position(|space| *space == Space::Empty)
            .map(|pos| pos + skip)
    }

    fn get_block_size(&self, start: usize) -> usize {
        let space = self.disk[start];
        self.disk
            .iter()
            .skip(start)
            .position(|s| *s != space)
            .unwrap_or_else(|| self.disk.len() - start)
    }

    fn get_block_info(&self, file_id: usize) -> (usize, usize) {
        let start = self
            .disk
            .iter()
            .position(|space| *space == Space::File(file_id))
            .unwrap();
        (start, self.get_block_size(start))
    }

    fn get_first_empty_space_block(&self, size: usize, max_index: usize) -> Option<usize> {
        let mut empty_space_index = self.get_first_empty_space_index(0);
        while let Some(i) = empty_space_index {
            if i >= max_index {
                return None;
            }
            let found_space = self.get_block_size(i);
            if found_space >= size {
                return Some(i);
            }
            empty_space_index = self.get_first_empty_space_index(i + size);
        }
        None
    }

    fn defragmentation(&mut self) {
        self.remove_empty_spaces_at_end();

        let mut empty_space_index = self.get_first_empty_space_index(0);
        while let Some(i) = empty_space_index {
            self.disk[i] = self.disk.pop().unwrap();
            self.remove_empty_spaces_at_end();
            empty_space_index = self.get_first_empty_space_index(i + 1);
        }
    }

    fn defragmentation_with_moving_whole_files(&mut self) {
        self.remove_empty_spaces_at_end();
        let max_file_id = self
            .disk
            .iter()
            .rfind(|&&space| space != Space::Empty)
            .map(|s| match s {
                Space::File(i) => i,
                _ => panic!("unreachable code"),
            })
            .unwrap();
        for i in (0usize..*max_file_id + 1).rev() {
            let (start, size) = self.get_block_info(i);
            let empty_block = self.get_first_empty_space_block(size, start);
            if let Some(empty_block) = empty_block {
                for j in empty_block..empty_block + size {
                    self.disk[j] = Space::File(i);
                }
                for j in start..start + size {
                    self.disk[j] = Space::Empty;
                }
            }
            self.remove_empty_spaces_at_end();
        }
    }
    fn checksum(&self) -> usize {
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
        cloned_self.checksum()
    }
    fn solve_second_part(&self) -> usize {
        let mut cloned_self = self.clone();
        cloned_self.defragmentation_with_moving_whole_files();
        cloned_self.checksum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "2333133121414131402";
    static SECOND_EXAMPLE: &str = "233313312141413140211";
    #[test]
    fn solve_first_part() {
        assert_eq!(Solver202409::from(EXAMPLE).solve_first_part(), 1928);
        assert_eq!(Solver202409::from(SECOND_EXAMPLE).solve_first_part(), 2132);
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(Solver202409::from(EXAMPLE).solve_second_part(), 2858);
        assert_eq!(Solver202409::from(SECOND_EXAMPLE).solve_second_part(), 2910);
    }
}
