use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: i8,
    y: i8,
    z: i8,
    w: i8
}

fn main() {
    let input : Vec<_> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "")
        .zip(0i8..)
        .flat_map(|(l, y)| l.chars().zip(0i8..).filter_map(move|(c, x)| if c == '#' { Some(Coord { x, y, z: 0, w: 0 }) } else { None }).collect::<Vec<_>>())
        .collect();

    let answer_one = (0..6)
        .fold(input.clone().into_iter().collect::<HashSet<_>>(), |active, _| {
            let mut adjacency : HashMap<Coord, usize> = HashMap::new();

            active.iter()
                  .flat_map(|c| (-1..=1).flat_map(move |x| (-1..=1).flat_map(move |y| (-1..=1).filter_map(move |z| if x != 0 || y != 0 || z != 0 { Some(Coord { x: c.x + x, y: c.y + y, z: c.z + z, w: 0 }) } else { None }))))
                  .for_each(|a| *adjacency.entry(a).or_insert(0) += 1);

            adjacency.iter()
                .filter_map(|(c, a)| if (active.contains(c) && *a == 2) || *a == 3 { Some(*c) } else { None })
                .collect()
        });

    println!("Part One: {}", answer_one.len());

    let answer_two = (0..6)
    .fold(input.into_iter().collect::<HashSet<_>>(), |active, _| {
        let mut adjacency : HashMap<Coord, usize> = HashMap::new();

        active.iter()
              .flat_map(|c| (-1..=1)
                .flat_map(move |x| (-1..=1)
                    .flat_map(move |y| (-1..=1)
                        .flat_map(move |z| (-1..=1)
                            .filter_map(move |w| if x != 0 || y != 0 || z != 0 || w != 0 { Some(Coord { x: c.x + x, y: c.y + y, z: c.z + z, w: c.w + w }) } else { None })))))
              .for_each(|a| *adjacency.entry(a).or_insert(0) += 1);

        adjacency.iter()
            .filter_map(|(c, a)| if (active.contains(c) && *a == 2) || *a == 3 { Some(*c) } else { None })
            .collect()
    });

    println!("Part Two: {}", answer_two.len());
}