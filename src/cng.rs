/*
converging number generator
*/
use std::ops::{Add, Sub};
use std::collections::HashSet;
use crate::std_rng;

#[derive(Clone)]
pub struct FShift {
    // index activation
    pub i_activation:usize,
    // index shift
    pub shift:usize
}

///////////////////////////

/*
function in ring:
- multiplication and addition capabilities

applying RFunc outputs f32

an RFunc derivative is another RFunc and the resultant
is adding the derivative to the RFunc.
*/
#[derive(Clone)]
pub struct RFunc {
    // real numbers 
    pub v: Vec<f32>,
    // 0->add|1->mult
    pub o: Vec<usize>    
}

pub fn build_RFunc(v:Vec<f32>,o:Vec<usize>) -> RFunc {
    assert!(v.len() == o.len());
    // TODO: hashset from iter
    let q:HashSet<usize> = HashSet::from_iter(o.clone());
    assert!(q.len() == 2);
    assert!(*(q.iter().max().unwrap()) == 1 as usize);
    assert!(*(q.iter().min().unwrap()) == 0 as usize);

    let mut rf = RFunc{v:v,o:o};
    rf.condense();
    rf
}

impl RFunc {

    /*
    inverts the struct instance by the following:
    +x -> -x
    *x -> * 1/x
    */ 
    pub fn invert(&mut self) -> RFunc {
        let mut r2 = self.clone();
        let l = r2.v.len();

        for i in 0..l {
            if self.o[i] == 0 {
                r2.v[i] = -1. * r2.v[i];  
            } else {
                r2.v[i] = if r2.v[i] != 0. {1.0 / r2.v[i]} else {r2.v[i]};
            }
        }
        r2
    }

    pub fn apply(&mut self,v:f32) -> f32 {
        let l = self.v.len();
        let mut v2 = v;
        for i in 0..l {
            if self.o[i] == 0 {
                v2 += self.v[i];
            } else {
                v2 *= self.v[i];
            }
        }
        v2
    }

    /*
    */ 
    pub fn condense(&mut self) {
        let mut i = 0;
        let mut l = self.v.len();

        while i < l - 1 {
            // check the next
            if self.o[i] == self.o[i + 1] {
                if self.o[i] == 0 {
                    self.v[i] = self.v[i] + self.v[i + 1];
                } else {
                    self.v[i] = self.v[i] * self.v[i + 1];
                }
                self.v.remove(i + 1);
                self.o.remove(i + 1);
                l = self.v.len();                
            } else {
                i += 1;
            }
        }
    }
}

impl Add for RFunc {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r3 = self.clone();
        r3.v.extend(&other.v);
        r3.o.extend(&other.o);
        r3.condense();
        r3
    }
}

/*
Integer Ring Function Deterministic Number Generator. 

A number generator that converges to a pattern of numbers.
*/
#[derive(Clone)]
pub struct IRFDNG {
    // ring function
    rf: Vec<RFunc>,
    // ring function shifter
    rfs: FShift,

    // ring function derivative
    // NOTE: does not have to be same length as rf
    rfd: Vec<RFunc>,
    // ring function derivative shifter
    rfds: FShift,

    // index of next
    i: usize,
    // index of rf for next 
    j: usize,
    // index of derivative for next
    k: usize,

    // range of values
    z: i32,
    range: (i32,i32)
}

pub fn build_IRFDNG(rf:Vec<RFunc>,rfs:FShift,rfd:Vec<RFunc>,rfds:FShift,
    z:i32,range:(i32,i32)) -> IRFDNG {
    assert!(z >= range.0 && z <= range.1);
    IRFDNG{rf:rf,rfs:rfs,rfd:rfd,rfds:rfds,i:0,j:0,k:0,z:z,range:range}
}

pub fn mod_in_range(x:i32,r:(i32,i32)) -> i32 {
    assert!(r.0 < r.1);
    if x >= r.0 && x <= r.1 {
        return x;
    }

    if x > r.1 {
        // get difference
        let d = (x - r.1) % (r.1 - r.0);
        return r.0 + d;
    } else {
        let d = (r.0 - x) % (r.1 - r.0);
        return r.1 - d; 
    }
}

/*
Q: the smallest integer i that must be stored?
A: for shifts x and y, if x divides y or y divides x,
    max(x,y), o.w. i. 
*/
impl IRFDNG {

    pub fn next(&mut self) -> i32 {
        let x = self.z;
        self.shift_rfunc();
        self.alter_rfunc();
        self.z = mod_in_range(self.rf[self.j].apply(self.z as f32).round() as i32,
                    self.range.clone());
        self.i += 1;
        x
    }

    pub fn shift_rfunc(&mut self) {
        if self.i % self.rfs.i_activation == 0 {
            let j = self.j; 
            self.j = (self.j + self.rfs.shift) % self.rf.len();
        }
    }

    pub fn alter_rfunc(&mut self) {
        if self.i % self.rfds.i_activation == 0 {
            // shift to next derivative
            self.k = (self.k + self.rfds.shift) % self.rfd.len();
            
            // modify rfunc
            self.rf[self.j] = self.rf[self.j].clone() + self.rfd[self.k].clone();
        }
    }
}

//////// sample RFunc vector 

pub fn sample_RFunc_vec1() -> Vec<RFunc> {

    let v1 = vec![3.,4.,5.,6.,1.2];
    let o1 = vec![0,1,1,1,0];
    let r1 = RFunc{v:v1,o:o1};

    let v2 = vec![31.,14.,0.5,6.];
    let o2 = vec![0,1,1,1];
    let r2 = RFunc{v:v2,o:o2};

    let v3 = vec![45.,1.];
    let o3 = vec![1,0];
    let r3 = RFunc{v:v3,o:o3};
    vec![r1,r2,r3]
}

pub fn sample_RFunc_vec2() -> Vec<RFunc> {

    let v1 = vec![0.3,12.];
    let o1 = vec![0,1];
    let r1 = RFunc{v:v1,o:o1};

    let v2 = vec![110.,31.,1.5];
    let o2 = vec![0,1,1];
    let r2 = RFunc{v:v2,o:o2};

    vec![r1,r2]
}

pub fn sample_RFunc_vec3() -> Vec<RFunc> {

    let v1 = vec![0.3,1.2];
    let o1 = vec![0,1];
    let r1 = RFunc{v:v1,o:o1};

    let v2 = vec![0.4,7.1,5.3];
    let o2 = vec![0,1,1];
    let r2 = RFunc{v:v2,o:o2};

    vec![r1,r2]
}

pub fn sample_RFunc_vec4() -> Vec<RFunc> {

    let v1 = vec![0.4,7.2];
    let o1 = vec![0,1];
    let r1 = RFunc{v:v1,o:o1};

    let v2 = vec![9.,4.5,7.3];
    let o2 = vec![0,1,1];
    let r2 = RFunc{v:v2,o:o2};

    let v3 = vec![19.,44.5,77.3];
    let o3 = vec![0,1,1];
    let r3 = RFunc{v:v3,o:o3};

    vec![r1,r2,r3]
}

/*
pub fn sample_FShift1() -> FShift {
    FShift{i_activation:4,shift:6} 
}

pub fn sample_FShift2() -> FShift {
    FShift{i_activation:3,shift:9} 
}
*/

pub fn sample_FShift1() -> FShift {
    FShift{i_activation:1,shift:2} 
}

pub fn sample_FShift2() -> FShift {
    FShift{i_activation:2,shift:1} 
}

/*
pub fn sample_IRFDNG() -> IRFDNG {
    build_IRFDNG(sample_RFunc_vec1(),sample_FShift1(),
        sample_RFunc_vec2(),sample_FShift2(),
        5,(-23,112))
}
*/

/*
pub fn sample_IRFDNG() -> IRFDNG {
    build_IRFDNG(sample_RFunc_vec1(),sample_FShift1(),
        sample_RFunc_vec2(),sample_FShift2(),
        5,(-200,1120))
}
*/

pub fn sample_IRFDNG() -> IRFDNG {
    build_IRFDNG(sample_RFunc_vec4(),sample_FShift1(),
        sample_RFunc_vec3(),sample_FShift2(),
        5,(-200,1120))
}

////////// std. random generators 

/*
RFunc with 2 <= x<= 10 operands, 
each operand is in range [-100,100]
*/ 
pub fn std_random_RFunc() -> RFunc {

    // random number of operands 
    let sl = std_rng::random_i32_in_range((2,10)) as usize;
    let mut v:Vec<f32> = Vec::new();
    let mut o:Vec<usize> = Vec::new();

    // get random oper(ands|ators)    
    for _ in 0..sl {
        let v_ = std_rng::random_f32_in_range((-100.,100.));
        v.push(v_);

        let o_ = std_rng::random_i32_in_range((0,1)) as usize;
        o.push(o_);
    }

    RFunc{v:v,o:o}
}


/*
i_activation in range (0,100)
shift in range (0,100)
*/
pub fn std_random_FShift() -> FShift {
    let s1 = std_rng::random_i32_in_range((0,100)) as usize;
    let s2 = std_rng::random_i32_in_range((0,100)) as usize;
    FShift{i_activation:s1,shift:s2}
}

/// generates a pseudo-range IRFDNG that outputs values 
/// in the range r
pub fn std_random_IRFDNG(r:(i32,i32)) -> IRFDNG {

    // each RFunc vec will have length 2 <= x <= 10
    let s1 = std_rng::random_i32_in_range((2,10));
    let s2 = std_rng::random_i32_in_range((2,10));

    let mut v1:Vec<RFunc> = Vec::new();
    let mut v2:Vec<RFunc> = Vec::new();

    for _ in 0..s1 {
        v1.push(std_random_RFunc());
    }

    for _ in 0..s2 {
        v2.push(std_random_RFunc());
    }

    let v = std_rng::random_i32_in_range(r.clone());
    build_IRFDNG(v1,sample_FShift1(),
        v2,sample_FShift2(),v,r)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__mod_in_range() {
        let r = (-23,51);
        let y = -102;
        let x = mod_in_range(y,r);
        assert!(x == 46);
    
        let y2 = 200;
        let x2 = mod_in_range(y2,r);
        assert!(x2 == -22);
    }

}