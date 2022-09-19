
//! database for Q.
//! logs responses of nodes, and sends them 
use ndarray::{arr1,arr2,Array1,Array2,Dim,s};
use crate::ans;
use std::collections::HashSet;
use std::fmt;

/// structure for containing Q's data
/// row is node
/// column is question
pub struct QData {
    /// rate of delegation
    pub w: Array2<usize>,
    /// rate of contradiction
    pub x: Array2<f32>,
    /// duplicates of question asked
    pub y: Array2<usize>,
    /// mean answers of node
    pub z: Array2<i32> 
}

pub fn build_QData(r:usize,c:usize) -> QData {
    let w: Array2<usize> = Array2::zeros((r,c));
    let x: Array2<f32> = Array2::zeros((r,c));
    let y: Array2<usize> = Array2::zeros((r,c));
    let z: Array2<i32> = Array2::zeros((r,c));
    QData{w:w,x:x,y:y,z:z} 
}

impl fmt::Display for QData {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s1 = format!("-- W\n{:?}\n",self.w);
        let mut s2 = format!("-- X\n{:?}\n",self.x);
        let mut s3 = format!("-- Y\n{:?}\n",self.y);
        let mut s4 = format!("-- Z\n{:?}\n",self.z);
        let mut q = "".to_string();
        q.push_str(&s1);
        q.push_str(&s2);
        q.push_str(&s3);
        q.push_str(&s4);
        write!(f, "{}", q)
    }
}

impl QData {

    /// 
    pub fn average_ans_to_question(&mut self,qi:usize,ansrange:(i32,i32)) -> i32 {
        let r1:Array1<f32> = self.z.slice(s![..,qi]).to_owned().into_iter().map(|q| q as f32).collect();
        let r2:Array1<f32> = self.y.slice(s![..,qi]).to_owned().into_iter().map(|q| q as f32).collect();
        let a:i32 = 0; 
        let s = r2.sum();

        // case: no questions asked, use average of range
        if s == 0. {
            return ((ansrange.1 + ansrange.0) as f32 / 2.).round() as i32;
        }

        ((r1 * r2).sum() / s).round() as i32
    }

    /// # arguments
    /// wanted_resp := None if Q has known answer else draw from QData 
    pub fn log_node_response(&mut self,srcidn: usize,nidns:HashSet<usize>,qi:usize, ansrange: (i32,i32),wanted_resp: Option<i32>,resp:i32) -> f32 {
        assert!(nidns.len() > 0);
        let r = if wanted_resp.is_none() {self.average_ans_to_question(qi,ansrange.clone())} else {wanted_resp.unwrap()};
        ////println!("logging node response");
        ////println!("Q answer: {}\tnode response: {}",r,resp);

        let l = nidns.len();
        // calculate contradiction
        let a = ans::invert_calculate_ans(ansrange,r,resp);
        
        // distribute contradiction among all pertinent nodes
        let da = a / nidns.len() as f32;
        for n in nidns.clone().into_iter() {
            self.mod_qdata_of_node(n,qi,da);
        }

        // modify delegate nodes count
        for n in nidns.into_iter() {
            if n == srcidn {
                continue;
            }
            let d = Dim((n,qi));
            self.w[d] += l;
        }

        let d = Dim((srcidn,qi));
        self.y[d] += 1; 

        // modify mean answer of node
        self.z[d] = ((self.z[d] as f32 * (self.y[d] - 1) as f32 + resp as f32) / 
                    (self.y[d] as f32).round()) as i32;
        da
    }

    pub fn mod_qdata_of_node(&mut self,ni:usize,qi:usize,c:f32) {
        let d = Dim((ni,qi));
        let mut q = self.x[d] * self.y[d] as f32 + c;
        let mut q2 = q / (self.y[d] as f32 + 1.);
        self.x[d] = q2;
    }

}