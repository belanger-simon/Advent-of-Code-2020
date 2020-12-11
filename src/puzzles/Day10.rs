use crate::puzzles::Puzzle;

struct Day10 { }

pub fn create() -> impl Puzzle {
    Day10 { }
}

impl Puzzle for Day10 {
    fn solve_part_one(&self, input: &Vec<String>) -> Option<String> {
        let mut adapters = parse_input(input);

        let mut diff_1 = 0;
        let mut diff_3 = 1;

        adapters.sort();

        for (l, n) in vec![0].iter().chain(adapters.iter()).zip(adapters.iter()) {
            match n - l {
                1 => diff_1 += 1,
                3 => diff_3 += 1,
                _ => { }
            }
        }

        Some((diff_1 * diff_3).to_string())
    }

    fn solve_part_two(&self, input: &Vec<String>) -> Option<String> {
        let mut adapters : Vec<u32> = parse_input(input);

        adapters.push(0);
        adapters.sort();
        adapters.push(adapters.last().unwrap() + 3);
        
        // Max cluster of input is 5 (e.g. 10 13 14 15 16 17 20)
        // Clusters of 1 (10 13 16) or 2 (10 13 14 17) don't have different combination since they are attached to the
        // Low / High bound. Cluster of 3 has 2 different permutations, Cluster of 4 has 4 and cluster of 5 has 7.
        let mut cluster_of_3 = 0;
        let mut cluster_of_4 = 0;
        let mut cluster_of_5 = 0;
        let mut cluster_size = 0;
        let mut last = 0;
        
        for j in adapters.iter() {
            if j - last == 3 {                
                match cluster_size {
                    3 => cluster_of_3 += 1,
                    4 => cluster_of_4 += 1,
                    5 => cluster_of_5 += 1,
                    x if x > 5 => panic!("Cluster > 5"),
                    _ => { }
                }
                
                cluster_size = 1;
            }
            else {
                cluster_size += 1;
            }

            last = *j;
        }

        Some((
            2i64.pow(cluster_of_3) *
            4i64.pow(cluster_of_4) *
            7i64.pow(cluster_of_5)
        ).to_string())
    }
}

fn parse_input(input: &[String]) -> Vec<u32> {
    input.iter()
        .map(|o| o.parse::<u32>().unwrap())
        .collect()
}
