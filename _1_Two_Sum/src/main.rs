use std::collections::HashMap;

fn main() {
    let a = vec![2, 7, 11, 15, 77, 99, 6, 4, 999, 34];
    let r = two_sum(a, 103);
    println!("{:?}", r);
}

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut index1 = 0;
//     let mut index2;
//     let ln = nums.len();
//     let vec1 = 'outer: loop {
//         index2 = index1 + 1;
//         loop {
//             if nums[index1] + nums[index2] == target {
//                 if index1 != index2 {
//                     break 'outer vec![index1 as i32, index2 as i32];
//                 }
//             }
//             if index2 < ln - 1 {
//                 index2 += 1;
//             } else {
//                 break;
//             }
//         }
//         if index1 < ln - 2 {
//             index1 += 1;
//         } else {
//             break vec![index1 as i32, index2 as i32];
//         }
//     };
//
//     vec1
// }

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     use std::collections::HashMap;
//     let mut map = HashMap::<i32, i32>::new();
//     println!("target = {}", target);
//     for (i, x) in nums.iter().enumerate() {
//         match map.get(x) {
//             Some(y) => {
//                 println!("i={},x= {}, y ={}", i, x, y);
//                 for (key, value) in map.iter() {
//                     print!("{}: {} || ", key, value);
//                 }
//                 return vec![i as i32, *y];
//             }
//             None => {
//                 map.insert(target - x, i as i32);
//                 println!("i={},x= {}, y =None", i, x);
//                 for (key, value) in map.iter() {
//                     print!("{}: {} || ", key, value);
//                 }
//             }
//         };
//     }
//     vec![]
// }
// //

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    // target = x1 + x2;  x2 = target - x1;  x1 != x2
    for (i, x1) in nums.iter().enumerate() {
        match map.get(x1) {
            Some(x2) => return vec![*x2, i as i32],
            None => map.insert(target - x1, i as i32),
        };
    }
    Vec::new()
}
