use crate::io_utils;

// 1 -> 3 -> 6 -> 10?
//   2    3    4?
//     1     1?
// [1]
// [3, 2]
// [6, 3, 1]
// [10, 4, 1, 0]
// ----- reverse case? ----
// [10]
// [6, -4]
// [3, -3, 1]
// [1, -2, 1, 0]
// [0, -1,  1, 0]

pub fn run() {
    let lines = io_utils::read_file_lines("inputs/d9.txt").unwrap();
    let mut next_sum = 0;
    for l in lines {
        let nums_it = l
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        let mut stack = DerivativeStack::new();
        for n in nums_it {
            advance_stack(&mut stack, n);
        }
        let next = predict_next(&stack);
        println!("next: {}", next);
        next_sum += next;
    }
    println!("next_sum: {}", next_sum);
}

type DerivativeStack = Vec<i32>;

fn advance_stack(stack: &mut DerivativeStack, next: i32) {
    let mut last_change = next;
    // update stack
    stack.iter_mut().for_each(|s| {
        let cur = *s; // copy
        *s = last_change; // update stack
        last_change -= cur; // get new diff: new - old
    });
    stack.push(last_change);
}

fn predict_next(stack: &DerivativeStack) -> i32 {
    stack.iter().sum()
}
