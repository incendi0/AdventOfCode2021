pub fn first_part(xs: &[i32]) -> u32 {
    xs.windows(2)
        .fold(0, |acc, t| if t[0] < t[1] { acc + 1 } else { acc })
}

pub fn second_part(xs: &[i32]) -> u32 {
    xs.windows(3).zip(xs.windows(3).skip(1)).fold(0, |acc, t| {
        let s1 = t.0.iter().sum::<i32>();
        let s2 = t.1.iter().sum::<i32>();
        if s1 < s2 {
            acc + 1
        } else {
            acc
        }
    })
}
