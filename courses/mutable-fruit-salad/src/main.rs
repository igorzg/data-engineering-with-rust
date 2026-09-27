fn main() {
    let fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad {:?}", fruit_salad);

    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    fruit_salad.push("figs");
    println!("Modified fruit salad {:?}", fruit_salad);
}
