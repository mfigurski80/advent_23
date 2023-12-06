use std::ops::Add;
use std::ops::Range;

/// Transpose range by constant value
pub fn transpose<T: Add<T, Output = T> + Copy>(r: &Range<T>, val: T) -> Range<T> {
    let lo = r.start + val;
    let hi = r.end + val;
    lo..hi
}

pub fn transpose_uns(r: &Range<usize>, val: isize) -> Range<usize> {
    let lo = r.start as isize + val;
    let hi = r.end as isize + val;
    lo as usize..hi as usize
}

/// return true if inner range bound is contained within outer range
pub fn bounds_contained<T: PartialOrd>(inner: &Range<T>, outer: &Range<T>) -> (bool, bool) {
    (outer.contains(&inner.start), outer.contains(&inner.end))
}

/// split target range into left and right based on partition value
pub fn partition_on(
    target: Range<usize>,
    val: usize,
) -> (Option<Range<usize>>, Option<Range<usize>>) {
    match (val > target.start, val < target.end) {
        (true, true) => (Some(target.start..val), Some(val..target.end)),
        (true, false) => (Some(target), None),
        (false, true) => (None, Some(target)),
        _ => panic!("target range is inverted"),
    }
}

/// split target range into left, center, and right based on partition range
pub fn partition_on_range(
    target: Range<usize>,
    val: &Range<usize>,
) -> (
    Option<Range<usize>>,
    Option<Range<usize>>,
    Option<Range<usize>>,
) {
    let (left, center) = partition_on(target, val.start);
    if center.is_none() {
        return (left, None, None);
    }
    let (center, right) = partition_on(center.unwrap(), val.end);
    (left, center, right)
}
