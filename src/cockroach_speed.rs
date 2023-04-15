// https://www.codewars.com/kata/55fab1ffda3e2e44f00000c6

pub fn cockroach_speed( s : f64 ) -> i64 {
    (s * 250.0/9.0) as i64
}


#[cfg(test)]
mod tests {
    use super::cockroach_speed;

    #[test]
    fn returns_expected() {
      assert_eq!(cockroach_speed(1.08), 30);
      assert_eq!(cockroach_speed(1.09), 30);
      assert_eq!(cockroach_speed(0.0), 0);
    }
}
