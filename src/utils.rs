pub fn substract_vec<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: std::cmp::PartialEq + Copy,
{
    let mut result = Vec::new();

    for &item in v1 {
        if !v2.contains(&item) {
            result.push(item);
        }
    }

    result
}
