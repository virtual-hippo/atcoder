// extern crate abclib;
// use abclib::gcd::GCD;
// println!("{}", GCD::gcd(12, 16));

pub trait GCD {
    fn gcd(m: Self, n: Self) -> Self;
}

impl GCD for u8 {
    fn gcd(x: u8, y: u8) -> u8 {
        let mut xy = (x, y);
        while xy.0 >= 1 && xy.1 >= 1 {
            if xy.0 < xy.1 {
                xy.1 %= xy.0;
            } else {
                xy.0 %= xy.1;
            }
        }
        if xy.0 >= 1 {
            xy.0
        } else {
            xy.1
        }
    }
}

impl GCD for u16 {
    fn gcd(x: u16, y: u16) -> u16 {
        let mut xy = (x, y);
        while xy.0 >= 1 && xy.1 >= 1 {
            if xy.0 < xy.1 {
                xy.1 %= xy.0;
            } else {
                xy.0 %= xy.1;
            }
        }
        if xy.0 >= 1 {
            xy.0
        } else {
            xy.1
        }
    }
}

impl GCD for u32 {
    fn gcd(x: u32, y: u32) -> u32 {
        let mut xy = (x, y);
        while xy.0 >= 1 && xy.1 >= 1 {
            if xy.0 < xy.1 {
                xy.1 %= xy.0;
            } else {
                xy.0 %= xy.1;
            }
        }
        if xy.0 >= 1 {
            xy.0
        } else {
            xy.1
        }
    }
}

impl GCD for u64 {
    fn gcd(x: u64, y: u64) -> u64 {
        let mut xy = (x, y);
        while xy.0 >= 1 && xy.1 >= 1 {
            if xy.0 < xy.1 {
                xy.1 %= xy.0;
            } else {
                xy.0 %= xy.1;
            }
        }
        if xy.0 >= 1 {
            xy.0
        } else {
            xy.1
        }
    }
}

impl GCD for usize {
    fn gcd(x: usize, y: usize) -> usize {
        let mut xy = (x, y);
        while xy.0 >= 1 && xy.1 >= 1 {
            if xy.0 < xy.1 {
                xy.1 %= xy.0;
            } else {
                xy.0 %= xy.1;
            }
        }
        if xy.0 >= 1 {
            xy.0
        } else {
            xy.1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_u8() {
        let result = GCD::gcd(4_u8, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_u16() {
        let result = GCD::gcd(4_u16, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_u32() {
        let result = GCD::gcd(4_u32, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_u64() {
        let result = GCD::gcd(4_u64, 12);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_usize() {
        let result = GCD::gcd(4_usize, 12);
        assert_eq!(result, 4);
    }
}
