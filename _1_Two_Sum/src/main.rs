fn main() {
    let a = vec![2, 7, 11, 15, 77, 99, 6, 4, 999, 34];
    let r = two_sum(a, 10);
    println!("{:?}", r);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index1 = 0;
    let mut index2;
    let ln = nums.len();
    let vec1 = 'outer: loop {
        index2 = index1 + 1;
        loop {
            if nums[index1] + nums[index2] == target {
                if index1 != index2 {
                    break 'outer vec![index1 as i32, index2 as i32];
                }
            }
            if index2 < ln - 1 {
                index2 += 1;
            } else {
                break;
            }
        }
        if index1 < ln - 2 {
            index1 += 1;
        } else {
            break vec![index1 as i32, index2 as i32];
        }
    };

    vec1
}
