// computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for ele in slice.iter() {
        sum += ele;
    }
    sum
}

// Deduplicates items in the input vector `vs`. Produces a vector containing
// the first instance of each distinct element of `vs`, preserving the 
// original order.
/*pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    
}*/

