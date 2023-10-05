use tests::prepare_config;

#[test]
fn test_config() {
    let cfg = prepare_config(123, 456);

    assert_eq!(cfg.min_value, 123);
    assert_eq!(cfg.max_value, 456);
}
