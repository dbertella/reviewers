use rand::seq::SliceRandom;

struct Reviewers<'a> {
    pool: Vec<Vec<&'a str>>,
}

impl<'a> Reviewers<'a> {
    fn new(members: Vec<Vec<&'a str>>) -> Self {
        Self { pool: members }
    }

    fn refresh_pool(&mut self, members: Vec<Vec<&'a str>>) {
        self.pool = members;
    }

    fn get_reviewer(&mut self, member: &'a str) -> Result<Vec<&'a str>, String> {
        let mut rng = rand::thread_rng();
        loop {
            self.pool.shuffle(&mut rng);
            let current_choice = self.pool.last().unwrap();

            if current_choice.contains(&member) {
                self.pool.shuffle(&mut rng);
            } else {
                let mut new_list = self.pool.pop().unwrap();
                new_list.push(member);
                return Ok(new_list);
            }
        }
    }

    fn build(&mut self, input: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
        let mut members = Vec::new();
        for row in input.iter() {
            let updated_list = self.get_reviewer(row[0]);
            members.push(updated_list.unwrap());
        }
        members
    }
}

fn main() -> csv::Result<()> {
    let input = vec![
        vec!["Oliver", "Ella", "Charlie"],
        vec!["Olivia", "Ava", "Ella"],
        vec!["George", "Mia", "Muhammad"],
        vec!["Amelia", "Muhammad", "Arthur"],
        vec!["Harry", "Sophia", "Emily"],
        vec!["Ava", "Charlie", "Oscar"],
        vec!["Noah", "Isabella", "Mia"],
        vec!["Isla", "Oscar", "Isabella"],
        vec!["Jack", "Harry", "Ava"],
        vec!["Emily", "Oliver", "Olivia"],
        vec!["Leo", "Olivia", "Amelia"],
        vec!["Mia", "Leo", "Grace"],
        vec!["Arthur", "Jack", "Sophia"],
        vec!["Isabella", "Emily", "Harry"],
        vec!["Muhammad", "Amelia", "George"],
        vec!["Sophia", "Noah", "Oliver"],
        vec!["Oscar", "Grace", "Jack"],
        vec!["Ella", "Isla", "Leo"],
        vec!["Charlie", "George", "Isla"],
        vec!["Grace", "Arthur", "Noah"],
    ];

    let mut reviewers = Reviewers::new(input.to_vec());

    let mut members = reviewers.build(&input);
    reviewers.refresh_pool(members);
    members = reviewers.build(&input);

    for m in members.iter() {
        println!("{}, {}, {}", m[0], m[3], m[4]);
    }
    Ok(())
}
