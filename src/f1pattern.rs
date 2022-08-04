/*
data struct 
*/ 
use crate::cng;
//use ndarray::{arr1,arr2,Array1,Array2,Dim,s};
use ndarray::{Array2,Dim};

#[derive(Clone)]
/// data is r x c 2-d matrix
/// row is question
/// column is answer
pub struct F1P {
    irfvec: Vec<cng::IRFDNG>,
    data: Array2<i32>
}

/// data is 2-d matrix, each row i corresponds to m responses
/// from node on question i  
pub fn build_std_random_F1P(data:Array2<i32>) -> F1P {
    let (r,c) = data.dim();
    let mut irfvec: Vec<cng::IRFDNG> = Vec::new(); 
    for i in 0..r {
        let mut irf = cng::std_random_IRFDNG((0,c as i32 - 1));
        irfvec.push(irf); 
    }

    F1P{irfvec:irfvec,data:data}
}

impl F1P {

    pub fn next(&mut self,qi:usize) -> i32 {
        let ni = self.irfvec[qi].next() as usize; 
        self.data[Dim((qi,ni))].clone()
    }

}