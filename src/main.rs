use my_lib::{MyVec, SortingAlgorithm, Sortable};

fn main() {
    let mut numbers = MyVec(vec![4, 2, 5, 1, 3, 4, 2, 0]);

    // sort using quick sort
    numbers.sort(SortingAlgorithm::QuickSort, |a, b| a < b);
    println!("After quick sort: {:?}", numbers.0);

    // sort using select sort
    numbers.sort(SortingAlgorithm::SelectSort, |a, b| a < b);
    println!("After select sort: {:?}", numbers.0);

    // sort using insert sort
    numbers.sort(SortingAlgorithm::InsertSort, |a, b| a > b);
    println!("After insert sort: {:?}", numbers.0);

    // sort using merge sort
    numbers.sort(SortingAlgorithm::MergeSort, |a, b| a > b);
    println!("After merge sort: {:?}", numbers.0);
}




// use my_lib::{MyVec, SortingAlgorithm, Sortable};

// fn main() {
//     // Example usage of sorting algorithms
//     let mut numbers = MyVec(vec![4, 2, 5, 1, 3, 4, 2, 0]);

//     // Sort using quick sort
//     numbers.sort(SortingAlgorithm::QuickSort);
//     println!("After quick sort: {:?}", numbers.0);

//     // Sort using select sort
//     numbers.sort(SortingAlgorithm::SelectSort);
//     println!("After select sort: {:?}", numbers.0);

//     // Sort using insert sort
//     numbers.sort(SortingAlgorithm::InsertSort);
//     println!("After insert sort: {:?}", numbers.0);

//     // Sort using merge sort
//     numbers.sort(SortingAlgorithm::MergeSort);
//     println!("After merge sort: {:?}", numbers.0);
// }


// mod my_lib;

// use my_lib::{MyVec, SortingAlgorithm, Sortable};

// fn main() {
//     let mut numbers = MyVec(vec![4, 2, 5, 1, 3]);

//     // Example comparison function for ascending order
//     let compare_asc = |a: &i32, b: &i32| a < b;

//     // Sort using quick sort algorithm
//     numbers.sort(SortingAlgorithm::QuickSort, &compare_asc);
    
//     // Similarly for other sorting algorithms
// }