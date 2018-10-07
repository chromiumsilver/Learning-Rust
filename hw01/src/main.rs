
fn main() {
    let array = [1,2,3,4,5];
    let sum = sum(&array);
    println!("{}", sum);
}

pub fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for ele in slice.iter() {
        sum += ele;
    }
    sum
}

