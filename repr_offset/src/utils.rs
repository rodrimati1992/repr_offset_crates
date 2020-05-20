/// A const-equivalent of `std::cmp::max::<usize>`
pub const fn min_usize(l: usize, r: usize) -> usize {
    [r, l][(l < r) as usize]
}
