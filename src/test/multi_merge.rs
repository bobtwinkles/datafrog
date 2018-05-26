use ::Relation;

#[test]
fn test_two_way_equal() {
    let r1 = Relation::from_vec(vec![0i32, 2]);
    let r2 = Relation::from_vec(vec![1, 3]);

    assert_eq!(
        Relation::multi_merge(vec![r1, r2]).elements,
        Relation::from_vec(vec![0i32, 1, 2, 3]).elements
    );
}

#[test]
fn test_two_way_unbalanced_1() {
    let r1 = Relation::from_vec(vec![0i32, 1, 2]);
    let r2 = Relation::from_vec(vec![3]);

    assert_eq!(
        Relation::multi_merge(vec![r1, r2]).elements,
        Relation::from_vec(vec![0i32, 1, 2, 3]).elements
    );
}

#[test]
fn test_two_way_unbalanced_2() {
    let r1 = Relation::from_vec(vec![3]);
    let r2 = Relation::from_vec(vec![0i32, 1, 2]);

    assert_eq!(
        Relation::multi_merge(vec![r1, r2]).elements,
        Relation::from_vec(vec![0i32, 1, 2, 3]).elements
    );
}

#[test]
fn test_multi_way_balanced() {
    let r1 = Relation::from_vec(vec![0i32, 2]);
    let r2 = Relation::from_vec(vec![1, 3]);
    let r3 = Relation::from_vec(vec![4, 5]);

    assert_eq!(
        Relation::multi_merge(vec![r1, r2, r3]).elements,
        Relation::from_vec(vec![0i32, 1, 2, 3, 4, 5]).elements
    );
}

#[test]
fn test_multi_way_unbalanced() {
    let r1 = Relation::from_vec(vec![0i32, 1, 2, 3, 4]);
    let r2 = Relation::from_vec(vec![]);
    let r3 = Relation::from_vec(vec![5]);

    assert_eq!(
        Relation::multi_merge(vec![r1, r2, r3]).elements,
        Relation::from_vec(vec![0i32, 1, 2, 3, 4, 5]).elements
    );
}
