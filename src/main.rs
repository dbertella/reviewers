use rand::seq::SliceRandom;
use std::collections::HashMap;

struct Reviewers<'a> {
    pool_1: Vec<&'a str>,
    pool_2: Vec<&'a str>,
    review_count: HashMap<&'a str, usize>,
}

impl<'a> Reviewers<'a> {
    fn new(members: &'a [&'a str]) -> Self {
        let mut review_count = HashMap::new();
        for member in members.iter() {
            review_count.insert(*member, 0);
        }
        Self {
            pool_1: Vec::from(members),
            pool_2: Vec::from(members),
            review_count,
        }
    }

    fn get_first_reviewer(&mut self, member: &'a str) -> &'a str {
        let mut rng = rand::thread_rng();
        loop {
            self.pool_1.shuffle(&mut rng);
            let reviewer = self.pool_1.last().unwrap();

            if *self.review_count.get(reviewer).unwrap() == 1 || *reviewer == member {
                self.pool_1.shuffle(&mut rng);
            } else {
                *self.review_count.get_mut(reviewer).unwrap() += 1;
                return self.pool_1.pop().unwrap();
            }
        }
    }

    fn get_second_reviewer(&mut self, members: [&'a str; 2]) -> &'a str {
        let mut rng = rand::thread_rng();
        loop {
            self.pool_2.shuffle(&mut rng);
            let reviewer = self.pool_2.last().unwrap();

            if *self.review_count.get(reviewer).unwrap() == 2 || members.contains(reviewer) {
                self.pool_2.shuffle(&mut rng);
            } else {
                *self.review_count.get_mut(reviewer).unwrap() += 1;
                return self.pool_2.pop().unwrap();
            }
        }
    }
}

fn main() -> csv::Result<()> {
    let data = [
        "Oliver", "Olivia", "George", "Amelia", "Harry", "Ava", "Noah", "Isla", "Jack", "Emily",
        "Leo", "Mia", "Arthur", "Isabella", "Muhammad", "Sophia", "Oscar", "Ella", "Charlie",
        "Grace",
    ];

    let mut reviewers = Reviewers::new(&data);
    let mut members = Vec::new();
    for record in data.iter() {
        let first_reviewer = reviewers.get_first_reviewer(record);
        members.push((record, first_reviewer));
    }

    for (i, _) in data.iter().enumerate() {
        let (member, first_reviewer) = members[i];
        println!(
            "{},{},{}",
            member,
            first_reviewer,
            reviewers.get_second_reviewer([member, first_reviewer])
        );
    }
    Ok(())
}
