


// Topic: Vectors

// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector

// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

struct Score {
    score: i32,
}

impl Score {
    fn new(score: i32) -> Self {
        Self {
            score
        }
    }

    fn print(&self) {
        match self.score {
            30 => println!("thirty"),
            _ => println!("{:?}", self.score),

        }
    }
}

fn main() {
    let scores = vec![
        Score::new(10),
        Score::new(20),
        Score::new(30),
        Score::new(40)
    ];

    for score in &scores {
        score.print();
    }
    println!("{:?}", scores.len());
}