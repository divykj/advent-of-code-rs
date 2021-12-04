pub(crate) mod input {
    use std::fs;

    pub fn read_numbers(filename: &str) -> Vec<u64> {
        fs::read_to_string(filename)
            .expect("Cannot read input.")
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect()
    }
}

pub(crate) mod output {
    #[derive(Debug)]
    pub enum Unsolved {
        Year,
        Day,
        Part,
    }
}
