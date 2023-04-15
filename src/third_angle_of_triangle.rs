// https://www.codewars.com/kata/5a023c426975981341000014

pub fn third_angle_of_triangle(a: u32, b: u32) -> u32 {
  180 - (a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(third_angle_of_triangle(30, 60), 90);
        assert_eq!(third_angle_of_triangle(60, 60), 60);
        assert_eq!(third_angle_of_triangle(43, 78), 59);
        assert_eq!(third_angle_of_triangle(10, 20), 150);
    }
}
