use include_url::include_raw_url;

include_raw_url!("https://raw.githubusercontent.com/Milesq/include-url/main/assets/add.rs");

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
