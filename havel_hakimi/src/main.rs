// https://www.reddit.com/r/dailyprogrammer/comments/bqy1cf/20190520_challenge_378_easy_the_havelhakimi/?utm_source=share&utm_medium=web2x

fn main() {
    if test_hh() {
        println!("Tests were successful")
    }
    else {
        println!("Tests were not successful")
    }
}

fn havel_hakimi(mut answers: Vec<i32>) -> bool {
    answers.sort();
    answers.reverse();
    loop {
        answers = answers.into_iter().filter(|&x| x != 0).collect();
        if answers.len() == 0 { return true };
        let n: i32 = answers.pop().unwrap();
        if n > answers.len() as i32 { return false };
        answers = answers.iter().map(|x| { x - 1 }).collect();
    }
}

fn test_hh() -> bool {
    let tests: [(Vec<i32>, bool); 12] = [
        (vec![5, 3, 0, 2, 6, 2, 0, 7, 2, 5], false),
        (vec![4, 2, 0, 1, 5, 0], false),
        (vec![3, 1, 2, 3, 1, 0], true),
        (vec![16, 9, 9, 15, 9, 7, 9, 11, 17, 11, 4, 9, 12, 14, 14, 12, 17, 0, 3, 16], true),
        (vec![14, 10, 17, 13, 4, 8, 6, 7, 13, 13, 17, 18, 8, 17, 2, 14, 6, 4, 7, 12], false),
        (vec![15, 18, 6, 13, 12, 4, 4, 14, 1, 6, 18, 2, 6, 16, 0, 9, 10, 7, 12, 3], false),
        (vec![6, 0, 10, 10, 10, 5, 8, 3, 0, 14, 16, 2, 13, 1, 2, 13, 6, 15, 5, 1], false),
        (vec![2, 2, 0], false),
        (vec![3, 2, 1], false),
        (vec![1, 1], true),
        (vec![1], false),
        (vec![], true)
    ];

    for entry in tests.iter().enumerate() {
        let (index, test) = entry;

        let (answers, result) = test;

        if havel_hakimi(answers.to_vec()) != *result { 
            println!("{}, {}", index, result);
            return false;
        };
    }

    true
}
