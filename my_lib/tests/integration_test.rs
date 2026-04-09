use my_lib::basics;

#[test]
fn it_adds_two_and_two() {
    let result = basics::add(2, 2);
    assert_eq!(result, 4);
}