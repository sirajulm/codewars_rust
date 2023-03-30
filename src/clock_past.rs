// https://www.codewars.com/kata/55f9bca8ecaa9eac7100004a/train/rust
use std::time::Duration;
use std::convert::TryInto;

pub fn past(h: i32, m: i32, s: i32) -> i32 {
  // simpler and cleverer would be
  // let ms = ((h * 60 + m) * 60 + s) * 1000;
  
  let h_to_s = Duration::from_secs((h * 60 * 60) as u64);  
  let m_to_s = Duration::from_secs((m * 60) as u64); 
  let s = h_to_s + m_to_s + Duration::from_secs(s as u64); 
  match s.as_millis().try_into() {
    Ok(ms) => ms,
    Err(_) => {
      println!("error converting number");
      return  0;
    },
      
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(past(0, 1, 1), 61000);
        assert_eq!(past(1, 1, 1), 3661000);
        assert_eq!(past(0, 0, 0), 0);
        assert_eq!(past(1, 0, 1), 3601000);
        assert_eq!(past(1, 0, 0), 3600000);
    }
}
