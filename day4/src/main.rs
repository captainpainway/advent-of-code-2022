use std::fs;

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let arr = input.split_terminator("\n").collect::<Vec<&str>>();
    let contains = find_contained_assignments(
        split_chores(arr.clone())
    );
    println!("{} assignment pairs contain another assignment pair.", contains);
    let overlaps = find_overlaps(
        split_chores(arr)
    );
    println!("{} assignment pairs overlap.", overlaps);
}

fn split_chores(chores: Vec<&str>) -> Vec<Vec<Assignment>> {
    chores.iter().map(|chore| {
        return chore.split(",").map(|assignment| {
            let a = assignment
                .split("-")
                .collect::<Vec<&str>>();
            return Assignment {
                start: a[0].parse::<u32>().unwrap(),
                end: a[1].parse::<u32>().unwrap(),
            };
        }).collect::<Vec<Assignment>>()
    }).collect::<Vec<Vec<Assignment>>>()
}

fn find_contained_assignments(assignments: Vec<Vec<Assignment>>) -> usize {
    assignments.into_iter().filter(|a| {
        return a[0].start <= a[1].start && a[0].end >= a[1].end ||
        a[1].start <= a[0].start && a[1].end >= a[0].end;
    }).collect::<Vec<Vec<Assignment>>>().len()
}

fn find_overlaps(assignments: Vec<Vec<Assignment>>) -> usize {
    assignments.into_iter().filter(|a| {
        return a[0].start >= a[1].start && a[0].start <= a[1].end ||
            a[0].end >= a[1].start && a[0].end <= a[1].end ||
            a[1].start >= a[0].start && a[1].start <= a[0].end ||
            a[1].end >= a[0].start && a[1].start <= a[0].end;
    }).collect::<Vec<Vec<Assignment>>>().len()
}