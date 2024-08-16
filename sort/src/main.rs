fn sort_by_ages(names: Vec<&str>, ages: Vec<i32>) -> Vec<&str>{
    let mut combine = names.iter()
                    .enumerate()
                    .map(|(i, name)| (*name, ages[i]))
                    .collect::<Vec<(&str, i32)>>();

    combine.sort_by_key(|combined| combined.1);

    combine.iter()
            .map(|combined| combined.0)
            .collect()
}


fn main() {
    let names: Vec<&str> = vec!["Person1", "Person2", "Person3", "Person4"];
    let ages: Vec<i32> = vec![15, 32, 29, 12];

    println!("{:#?}", sort_by_ages(names, ages))
}
