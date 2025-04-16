#![no_std]

use core::array;
pub fn i4delt<T>(a: impl Iterator<Item = T> + Clone) -> impl Iterator<Item = (usize, T)> + Clone {
    let a = a.enumerate();
    let mut x: [_; 4] = array::from_fn(|i| a.clone().filter(move |a| a.0 % 4 == i));
    x.into_iter().flatten().scan(0usize, |s, (a, x)| {
        let d = a.wrapping_sub(*s);
        *s = a;
        Some((d, x))
    })
}
