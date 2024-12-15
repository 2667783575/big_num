use std::fmt;
use std::fmt::Debug;
use std::ops;
pub struct SimpleFloat {
    seq: Vec<bool>,
    upper_num: usize,
    lower_num: usize,
}

impl SimpleFloat {
    pub fn new(upper_num: usize, lower_num: usize) -> SimpleFloat {
        SimpleFloat {
            seq: Vec::new(),
            upper_num,
            lower_num,
        }
    }
    pub fn build(str: &str) -> SimpleFloat {
        let vec: Vec<char> = str.trim().chars().collect();
        let mut v: Vec<bool> = Vec::new();
        v.resize(vec.len(), false);
        let mut lower_num = 0;
        let mut bit = false;
        for i in 0..vec.len() {
            if vec[vec.len() - i - 1].eq(&'.') {
                lower_num = i;
                bit = true;
            } else if !bit {
                let o: u32 = vec[vec.len() - i - 1]
                    .to_digit(2)
                    .expect("the str should be valid!");
                v[i] = 1 == o;
            } else {
                let o: u32 = vec[vec.len() - i - 1]
                    .to_digit(2)
                    .expect("the str should be valid!");
                v[i - 1] = 1 == o;
            }
        }
        let mut len = vec.len() - lower_num;
        if bit {
            len -= 1;
            v.pop();
        }

        SimpleFloat {
            seq: v,
            upper_num: len,
            lower_num,
        }
    }
}

impl ops::Add<SimpleFloat> for SimpleFloat {
    type Output = SimpleFloat;
    fn add(self, add_num: SimpleFloat) -> SimpleFloat {
        let (upper_more, upper_less) = if self.upper_num > add_num.upper_num {
            (&self, &add_num)
        } else {
            (&add_num, &self)
        };

        let (lower_more, lower_less) = if self.lower_num > add_num.lower_num {
            (&self, &add_num)
        } else {
            (&add_num, &self)
        };

        let mut finally_up_bit: bool = false;
        let mut up_bit: bool = false;
        let mut v: Vec<bool> = Vec::new();
        v.resize(upper_more.upper_num + lower_more.lower_num + 1, false);
        for (i, cur) in v
            .iter_mut()
            .enumerate()
            .take(upper_more.upper_num + lower_more.lower_num + 1)
        {
            if i < lower_more.lower_num - lower_less.lower_num {
                *cur = lower_more.seq[i];
            } else if i < lower_more.lower_num {
                let a = lower_less.seq[i - (lower_more.lower_num - lower_less.lower_num)];
                let b = lower_more.seq[i];
                *cur = a ^ b ^ up_bit;
                up_bit = (a & b) | (a & up_bit) | (b & up_bit);
            } else if i < lower_more.lower_num + upper_less.upper_num {
                if lower_more.upper_num > lower_less.upper_num {
                    let a = lower_more.seq[i];
                    let b = lower_less.seq[i - (lower_more.lower_num - lower_less.lower_num)];
                    *cur = a ^ b ^ up_bit;
                    up_bit = (a & b) | (a & up_bit) | (b & up_bit);
                }
                //If lower_more is alse upper_more.
                else {
                    let a = lower_less.seq[i];
                    let b = lower_more.seq[i - (lower_more.lower_num - lower_less.lower_num)];
                    *cur = a ^ b ^ up_bit;
                    up_bit = (a & b) | (a & up_bit) | (b & up_bit);
                } //If lower_more is upper_less
            } else if i < lower_more.lower_num + upper_more.upper_num {
                if lower_more.upper_num > lower_less.upper_num {
                    let a = lower_more.seq[i];
                    let b = false;
                    *cur = a ^ b ^ up_bit;
                    up_bit = (a & b) | (a & up_bit) | (b & up_bit);
                }
                //If lower_more is alse upper_more.
                else {
                    let a = lower_less.seq[i - (lower_more.lower_num - lower_less.lower_num)];
                    let b = false;
                    *cur = a ^ b ^ up_bit;
                    up_bit = (a & b) | (a & up_bit) | (b & up_bit);
                } //If lower_more is upper_less
            } else {
                *cur = up_bit;
                finally_up_bit = up_bit;
            }
        }
        let mut up = upper_more.upper_num;
        let low = lower_more.lower_num;
        if finally_up_bit {
            up += 1;
        }

        SimpleFloat {
            seq: v,
            upper_num: up,
            lower_num: low,
        }
    }
}

impl PartialEq<SimpleFloat> for SimpleFloat {
    fn eq(&self, other: &SimpleFloat) -> bool {
        self.seq.eq(&other.seq)
            && self.upper_num == other.upper_num
            && self.lower_num == other.lower_num
    }
}

impl Debug for SimpleFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleFloat")
            .field("sequence", &self.seq)
            .field("upper_num", &self.upper_num)
            .field("lower_num", &self.lower_num)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_add() {
        let a = SimpleFloat {
            seq: vec![false, true, true, true, false],
            upper_num: 3,
            lower_num: 2,
        };
        let b = SimpleFloat {
            seq: vec![false, true, true, true, false],
            upper_num: 2,
            lower_num: 3,
        };
        let c = a + b;
        assert_eq!(c.seq, vec![false, true, false, true, false, true, false]);
        assert_eq!(
            SimpleFloat::build("1.1") + SimpleFloat::build("1.0"),
            SimpleFloat::build("10.1")
        );
    }
    #[test]
    fn test_build() {
        let a = SimpleFloat::build("11.11");
        let b = SimpleFloat {
            seq: vec![true, true, true, true],
            upper_num: 2,
            lower_num: 2,
        };
        let c = SimpleFloat::build("1.111");
        let d = SimpleFloat {
            seq: vec![true, true, true, true],
            upper_num: 1,
            lower_num: 3,
        };
        let x = SimpleFloat::build("10.100101");
        let y = SimpleFloat {
            seq: vec![true, false, true, false, false, true, false, true],
            upper_num: 2,
            lower_num: 6,
        };
        let m = SimpleFloat::build("1.0");
        let n = SimpleFloat {
            seq: vec![false, true],
            upper_num: 1,
            lower_num: 1,
        };
        assert_eq!(a, b);
        assert_eq!(c, d);
        assert_eq!(x, y);
        assert_eq!(m, n);
        let x = SimpleFloat::build("10.1");
        let y = SimpleFloat {
            seq: vec![true, false, true],
            upper_num: 2,
            lower_num: 1,
        };
        assert_eq!(x, y);

        let x = SimpleFloat::build("1.1");
        let y = SimpleFloat {
            seq: vec![true, true],
            upper_num: 1,
            lower_num: 1,
        };
        assert_eq!(x, y);
    }
}
