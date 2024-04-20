use my_lib_akniet::{MyVec, SortingAlgorithm, Sortable};

// #[derive(Debug, PartialEq)]

// struct Person {
//     name: String,
//     age: u32,
// }
// impl PartialOrd for Person {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.age.partial_cmp(&other.age)
//     }
// }

// impl Clone for Person {
//     fn clone(&self) -> Self {
//         Person {
//             name: self.name.clone(),
//             age: self.age.clone(),
//         }
//     }
// }

fn main() {
    let mut strings = MyVec(vec!["blockchain", "txt", "economy", "art"]);
    let mut numbers = MyVec(vec![4, 7, -5, -9, 0, 100]);

    //STRINGS
    // sort using quick sort
    strings.sort(SortingAlgorithm::QuickSort, |a, b| a < b);
    println!("After quick sort: {:?}", strings.0);

    // sort using select sort
    strings.sort(SortingAlgorithm::SelectSort, |a, b| a > b);
    println!("After reverse select sort: {:?}", strings.0);

    //NUMBERS
    // sort using insert sort
    numbers.sort(SortingAlgorithm::InsertSort, |a, b| a < b);
    println!("After insert sort: {:?}", numbers.0);

    // sort using merge sort
    numbers.sort(SortingAlgorithm::MergeSort, |a, b| a > b);
    println!("After reverse merge sort: {:?}", numbers.0);


    // let mut people = MyVec(vec![
    //     Person { name: "Alice".to_string(), age: 30 },
    //     Person { name: "Bob".to_string(), age: 25 },
    //     Person { name: "Charlie".to_string(), age: 35 },
    // ]);
    //STRUCTS
    // sort the vector of Person structs by age using quick sort
    // people.sort(SortingAlgorithm::QuickSort, |a, b| a < b);
    // println!("After sorting by age (quick sort): {:?}", people.0);

}

