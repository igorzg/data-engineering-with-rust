use challange_week3::greedy_coin_change;
#[test]
fn test_greedy_coin_change() {
    assert_eq!(greedy_coin_change(1), vec![1]);
    assert_eq!(greedy_coin_change(5), vec![5]);
    assert_eq!(greedy_coin_change(10), vec![10]);
    assert_eq!(greedy_coin_change(25), vec![25]);
    assert_eq!(greedy_coin_change(26), vec![25, 1]);
    assert_eq!(greedy_coin_change(27), vec![25, 1, 1]);
    assert_eq!(greedy_coin_change(28), vec![25, 1, 1, 1]);
    assert_eq!(greedy_coin_change(29), vec![25, 1, 1, 1, 1]);
    assert_eq!(greedy_coin_change(30), vec![25, 5]);
    assert_eq!(greedy_coin_change(31), vec![25, 5, 1]);
}