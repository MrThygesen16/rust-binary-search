pub(crate) fn search(vector: Vec<i32>, target: i32) -> Option<i32> {

    if vector.len() == 0 {return None}

    let mid = vector.len()/2;

    if vector[mid] == target {return Some(target)}

    let mut start = 0;

    let mut end = vector.len()-1;

    if vector[mid] > target {
        end = mid;
    } else if vector[mid] < target {
        start = mid;
    }    

    let new_v = &vector[start..end];

    search(new_v.to_vec(), target)
}

#[test]
fn test_empty_list() {
    let vec1 = vec![];
    assert_eq!(None, search(vec1, 100));
}

#[test]
fn test_one_item_list() {
    let vec1 = vec![0];
    assert_eq!(Some(0), search(vec1, 0));
}

#[test]
fn test_simple_list_1() {
    let vec1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(Some(1), search(vec1, 1));
}

#[test]
fn test_simple_list_2() {
    let vec1 = vec![17, 18, 21, 23, 29, 41, 45];
    assert_eq!(Some(29), search(vec1, 29));
}

#[test]
fn test_target_not_in_list() {
    let vec1 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_ne!(Some(11), search(vec1, 11));
}