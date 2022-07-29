/*
file contains custom struct for prng
*/
use std::ops::{Add, Sub};

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
    RFunc{v:v,o:o}
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
Integer Ring Function Deterministic Number Generator
*/ 
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
    assert(z >= range.0 && z <= range.1);
    IRFDNG{rf:rf,rfs:rfs,rfd:rfd,rfds:rfds,i:0,j:0,k:0,z:z,range:range}
}

// CAUTION:
pub fn mod_in_range(x:i32,r:(i32,i32)) -> i32 {
    assert(r.0 < r.1);
    if x >= r.0 && x <= r.1 {
        return x;
    }

    if x > r.1 {
        return r.0 + (x - r.1);
    } else {
        return r.1 - (r.0 - x); 
    }
}

/*
Q: the smallest integer i that must be stored?
A: for shifts x and y, if x divides y or y divides x,
    max(x,y), o.w. i. 
*/
impl IPRNG {

    pub fn next(&mut self) {
        let x = self.z;
        self.shift_rfunc();
        self.alter_rfunc();
        self.z = mod_in_range(self.rf[self.j].apply(self.z),self.range.clone());
        x
    }

    pub fn shift_rfunc(&mut self) {
        if self.i % self.rfs.i_activation == 0 {
            self.j = (self.j + self.rfs.shift) % self.rf.len();
        }
    }

    pub fn alter_rfunc(&mut self) {
        if self.i % self.rfds.i_activation == 0 {
            // modify rfunc
            self.rf[self.j] = self.rf[self.j] + self.rfd[self.k];

            // shift to next derivative
            self.k = (self.k + self.rfds.shift) % self.rfd.len();
        }
    }
}