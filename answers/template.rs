//! YEAR:   {{YEAR}}
//! DAY:    {{DAY}}

use crate::prelude::*;
use std::io::BufRead;

impl Answers for Day {
    fn new(input: PathBuf) -> Self {
        Day { input }
    }

    fn read(&self) -> BufReader<File> {
        let file = File::open(self.input.to_owned()).unwrap();
        return BufReader::new(file);
    }

    fn part_one(&self) -> Result<String, Box<dyn Error>>  {
        let reader = self.read();
        for line in reader.lines() {
            let read_line = line.unwrap();
        // Answer logic here
        }
        Ok("".into())
    }

    fn part_two(&self) -> Result<String, Box<dyn Error>> {
        let reader = self.read();
        for line in reader.lines() {
            let read_line = line.unwrap();
        // Answer logic here
        }
        Ok("".into())
    }
}