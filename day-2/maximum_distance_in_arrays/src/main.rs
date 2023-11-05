impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let n = arrays.len();
        let mut min = arrays[0][0];
        let mut max = arrays[0][arrays[0].len()-1];
        let mut ans = 0;

        for i in 1..n {
            let m = arrays[i].len();
            ans = ans.max(arrays[i][m-1] - min).max(max - arrays[i][0]);
            min = min.min(arrays[i][0]);
            max = max.max(arrays[i][m-1]);
        }

        ans
    }
}

fn main() {
    let arrays: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3, 4]];
    let result = Solution::max_distance(arrays);
    println!("Maximum Distance: {}", result);
}