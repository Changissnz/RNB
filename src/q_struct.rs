use crate::rdata;
use crate::seq_encoder;
use crate::ans; 
use crate::std_rng;
use crate::qsbf;
use ndarray::{arr1,arr2,Array,Array1,Array2,s,ScalarOperand};
use std::collections::HashSet;
use std::ops::Add;
use num_traits::identities::Zero;

/// convert arr1 index to arr2 index
pub fn arr1_index_to_arr2_index(i:usize,d:(usize,usize)) -> (usize,usize) {
    let mut i_:usize = i;
    let mut r:usize = 0;
    let mut c:usize = 0;

    while i_ >= d.1 {
        i_ -= d.1;
        r += 1;
    }

    (r,i_)
}

pub fn dead_node_filter<T>(a: Array2<T>,dead_nodes:HashSet<usize>,fv:T) -> Array2<T>
where T:Clone + Default + Add<Output=T> + Zero + ScalarOperand {
    let mut a2 = a.clone();
    let x:Array1<T> = Array1::zeros(a.dim().1) + fv;
    for d in dead_nodes.into_iter() {
        let mut b = a2.slice_mut(s![d,..]); 
        b.assign(&(x.clone()));
    }
    a2
}

/// question struct
#[derive(Clone)]
pub struct Q {
    // answer to question 
    pub qa: Option<i32>,
    pub ans_range: (i32,i32)
}

pub struct QStruct {
    pub qs: Vec<Q>,
    pub rd: rdata::QData,

    /// fix type 1 and type 2 filters
    pub f2_nodes: HashSet<usize>,

    // fuel level
    pub c: i32,

    // register dead nodes
    pub dead_nodes:HashSet<usize>
}

pub fn build_QStruct(qs:Vec<Q>,r:usize,c:i32) -> QStruct {
    let rd = rdata::build_QData(r,qs.len());
    QStruct{qs:qs,rd:rd,f2_nodes:HashSet::new(),c:c,dead_nodes:HashSet::new()}
}

impl QStruct {

    pub fn response_to_nodeset(&mut self,srcidn: usize, ns:HashSet<usize>,qi:usize,nodeset_ans:i32) {
        // log the response into data
        self.rd.log_node_response(srcidn,ns,qi,self.qs[qi].ans_range.clone(),self.qs[qi].qa.clone(),nodeset_ans);
    }

    /// method used in the case of known and unknown     
    pub fn ans_to_q(&mut self,qi:usize) -> i32 {

        // case: QStruct has a known answer
        if !self.qs[qi].qa.is_none() {
            return self.qs[qi].qa.clone().unwrap();
        }

        // case: QStruct does not have known answer
        self.rd.average_ans_to_question(qi,self.qs[qi].ans_range.clone())
    }

    pub fn ans_vec(&mut self) -> Array1<i32> {
        let l = self.qs.len();
        (0..l).into_iter().map(|x| self.ans_to_q(x)).collect()
    }

    /// # description
    /// calculates how QStruct will move:
    /// # return
    /// [0] target node and question pair
    /// [1] target node for F1 and QStruct fuel change
    pub fn one_move(&mut self) -> (Option<(usize,usize)>,Option<(usize,i32)>) {
        let mut nq = self.priority_nq_pair();
        let av = self.ans_vec();

        // fetch filtered delegate matrix
        let w = self.filtered_delegate_matrix();
        let n = qsbf::qbot_function_1(self.rd.z.clone(),self.rd.w.clone(),
            self.ans_vec(),self.f2_nodes.clone());
        (nq,n) 
    }

    /// marks any delegate matrix row as 0 if it is present in f2_nodes
    pub fn filtered_delegate_matrix(&mut self) -> Array2<usize> {
        let (r,c) = self.rd.w.dim();
        let mut dm:Array2<usize> = Array2::zeros((r,c));        
        let zs: Array1<usize> = Array1::zeros(c); 
        for i in 0..r {
            if self.f2_nodes.contains(&i) {
                continue; 
            }
            let mut b = dm.slice_mut(s![i,..]); 
            b.assign(&(zs.clone()));
        }
        dm
    }

    /// # description
    /// chooses the highest priority (node,question) pair
    /// by the following procedure:
    /// 1. if there exists an unanswered (node,question) pair, output it.
    /// 2. output the (node,question) pair of highest contradiction
    ///
    /// function outputs: 
    ///     (node idn,question idn)
    pub fn priority_nq_pair(&mut self) -> Option<(usize,usize)> { 
        // case 
        let x = self.random_unanswered_nq_pair(); 
        if !x.is_none() {
            return x;
        }
        self.max_contra_nq_pair() 
    } 

    /// max contradicting (node,question) pair is index (r,c)
    /// of (QData.x * QData.y)|(QData.x * QData.w)  
    pub fn max_contra_nq_pair(&mut self) -> Option<(usize,usize)> {
        let (r,c) = self.rd.x.dim();
        // case: all nodes are dead
        if self.dead_nodes.len() == r {
            return None;
        }

        // calculate x * y
            // convert y to f32
        let y = dead_node_filter(self.rd.y.clone(),self.dead_nodes.clone(),0); 
        let y2:Vec<f32> = y.clone().into_iter().map(|x| x as f32).collect();
        let y_:Array2<f32> = Array::from_shape_vec((r,c),y2).unwrap();
        let xy = self.rd.x.clone() * y_;

        // calculate x * w
            // convert w to f32
        let w = dead_node_filter(self.rd.w.clone(),self.dead_nodes.clone(),0); 
        let w2:Vec<f32> = w.clone().into_iter().map(|x| x as f32).collect();
        let w_:Array2<f32> = Array::from_shape_vec((r,c),w2).unwrap();
        let xw = self.rd.x.clone() * w_; 

        // determine max of x * y
        let (i,m1) = xy.into_iter().enumerate().fold((0,f32::MIN),
            |x,x2| if x.1 < x2.1 {x2} else {x}); 

        // determine max of x * w
        let (i2,m2) = xw.into_iter().enumerate().fold((0,f32::MIN),
            |x,x2| if x.1 < x2.1 {x2} else {x}); 

        // output index of max
        if m1 > m2 {
            return Some(arr1_index_to_arr2_index(i,(r,c)));
        }
        Some(arr1_index_to_arr2_index(i2,(r,c)))
    }
    

    /// choose random (node,question) pair in QData.y that is 0  
    pub fn random_unanswered_nq_pair(&mut self) -> Option<(usize,usize)> {
        let mut qi: Vec<usize> = Vec::new();
        let (r,c) = self.rd.y.dim();

        // collect all questions with  >= 1 nodes that did not answer
        for i in 0..c {
            let r2:Array1<usize> = self.rd.y.slice(s![..,i]).to_owned();
            let r3:Array1<usize> = r2.into_iter().enumerate().filter(|x| x.1 == 0).map(|x| x.0).collect(); 
            if r3.len() > 0 {
                qi.push(i);
            }
        }

        if qi.len() == 0 {
            return None;
        }

        let qic = std_rng::random_i32_in_range((0,qi.len() as i32 - 1)) as usize;
        
        // collect nodes that did not answer question
        let mut ni2:Array1<usize> = self.rd.y.slice(s![..,qi[qic]]).to_owned().clone();
        let ni:Array1<usize> = ni2.clone().into_iter().enumerate().filter(|x| x.1 == 0).map(|x| x.0).collect(); 
        let nic = std_rng::random_i32_in_range((0,ni.len() as i32 - 1)) as usize;
        Some((ni[nic],qi[qic]))
    } 

    
}

pub fn sample_QStruct1() -> QStruct {
    let q0 = Q{qa:Some(50),ans_range:(0,100)};
    let q1 = Q{qa:None,ans_range:(-80,80)};
    let q2 = Q{qa:Some(10),ans_range:(-50,25)};
    let q3 = Q{qa:Some(-25),ans_range:(-100,0)};
    let q4 = Q{qa:Some(6),ans_range:(0,10)};

    build_QStruct(vec![q0,q1,q2,q3,q4],11,2000)
}