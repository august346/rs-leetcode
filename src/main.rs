mod l001;
mod l002;
mod l003;
mod l004;
mod l005;
mod l006;
mod l007;
mod l008;
mod l009;
mod l010;
mod l011;
mod l012;
mod l013;
mod l014;

fn main() {
    l014::Solution::longest_common_prefix(vec!["ab","a"].into_iter().map(|s| s.to_string()).collect());
    println!("Hello, world!");
}
