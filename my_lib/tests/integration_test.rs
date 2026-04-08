use my_lib::add;

#[test]
fn it_adds_two_and_two() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}