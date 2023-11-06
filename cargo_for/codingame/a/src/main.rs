fn main() {
    let value = vec![1, 2, 3, 4, 5];

    for element in value.iter().rev().skip(1).rev() {
        println!("Element: {}", element);
    }
}




