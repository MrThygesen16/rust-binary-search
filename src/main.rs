mod binary_search;

fn main() {
    let vec1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = binary_search::search(vec1, 11);

    println!("{:?}", result);
}

