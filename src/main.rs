use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    iter::FromIterator,
};

const MAX_RETRY: usize = 50;

struct Reviewers {
    pool: Vec<Vec<String>>,
    retry: usize,
}

impl Reviewers {
    fn new(members: &Vec<Vec<String>>) -> Self {
        Self {
            pool: members.to_vec(),
            retry: 0,
        }
    }

    fn refresh_pool(&mut self, members: &Vec<Vec<String>>) {
        self.pool = members.to_vec();
    }

    fn get_reviewer(&mut self, member: String) -> Option<Vec<String>> {
        let mut rng = rand::thread_rng();
        while self.retry < MAX_RETRY {
            self.pool.shuffle(&mut rng);
            let current_choice = self.pool.last().unwrap();

            if current_choice.contains(&member) {
                self.pool.shuffle(&mut rng);
                self.retry += 1;
            } else {
                let mut new_list = self.pool.pop().unwrap();
                new_list.push(member);
                return Some(new_list);
            }
        }
        None
    }

    fn build(&mut self, input: &Vec<Vec<String>>) -> Option<Vec<Vec<String>>> {
        let mut members = Vec::new();
        for row in input.iter() {
            if let Some(updated_list) = self.get_reviewer(row[0].to_string()) {
                members.push(updated_list)
            } else {
                return None;
            }
        }
        Some(members)
    }
}

fn main() {
    let mut data = Vec::new();
    let f = File::open("reviewers.txt").expect("Failed to read the file");
    let f = BufReader::new(f);
    for line in f.lines() {
        data.push(Vec::from_iter(line.unwrap().split(",").map(String::from)));
    }
    let mut reviewers = Reviewers::new(&data);
    let mut members = reviewers
        .build(&data)
        .expect("Reload, didn't find a possible solutions");
    reviewers.refresh_pool(&members);
    members = reviewers
        .build(&data)
        .expect("Reload, didn't find a possible solutions");

    for mut m in members {
        println!("{} -> {},{}", m.pop().unwrap(), m.pop().unwrap(), m[0]);
    }
}
