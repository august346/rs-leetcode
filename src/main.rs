mod l1;
mod l2;
mod l3;
mod l4;
mod l5;
mod l6;
mod l7;
mod l8;
mod l9;
mod l10;

fn main() {
    l10::Solution::is_match("bbcacbabbcbaaccabc".to_string(), "b*a*a*.c*bb*b*.*.*".to_string());
    println!("Hello, world!");
}
