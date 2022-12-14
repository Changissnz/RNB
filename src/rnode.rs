//! contains Respondent Network Bot node. 
use crate::df; 
use crate::rndb;
use crate::ans;
use crate::f1pattern;
use ndarray::{Array2,Dim};

use std::collections::HashMap;
use std::fmt;

/// # description
/// calculates the default F1 answer pattern
pub fn default_F1_anspattern(r: &mut RNBNode,a: &mut ans::Ansbox,qrvec:Vec<(i32,i32)>) -> f1pattern::F1P {

    // gather 6 answers to each question 
    let l = qrvec.len();
    let mut x: Array2<i32> = Array2::zeros((l,6));
    for i in 0..l {
        for j in 0..6 {
            let y = (*r).indep_ans_to_q(a,j,qrvec[i].clone());
            x[Dim((i,j))] = y;
        }
    }

    // form the F1P
    f1pattern::build_std_random_F1P(x)
}

/// the node class used for Respondent Network Bot. 
#[derive(Clone)]
pub struct RNBNode {
    // identifier
    pub idn: usize,

    // delegator function 
    pub db: rndb::RNDB,
    pub neighbors: Vec<usize>,

    // resistance value:
    // when resistance falls below 0,
    // struct instance will contradict its objective
    pub resistance:f32,
    pub f1: Option<f1pattern::F1P>
}

pub fn build_RNBNode(idn:usize,db:rndb::RNDB,neighbors:Vec<usize>,resistance:f32) -> RNBNode {
    assert!(resistance > 0.);
    RNBNode{idn:idn,db:db,neighbors:neighbors,resistance:resistance,f1:None}
}

impl fmt::Display for RNBNode {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut q = &format!("node {}\nneighbors {:?}\nresistance {}",self.idn,self.neighbors,self.resistance);
        write!(f, "{}", q)
    }
}


impl RNBNode {

    /// # description
    /// answers to q by one of the following:
    /// 1. independent node answer.
    /// 2. fix-F1 answer.
    pub fn ans_to_q(&mut self,a: &mut ans::Ansbox,qi:usize,qr:(i32,i32)) -> i32 {
        if self.f1.is_none() {
            return self.indep_ans_to_q(a,qi,qr); 
        }
        let mut x = self.f1.clone().unwrap();
        let y = x.next(qi);
        self.f1 = Some(x);
        y
    }

    /// # description
    /// independent answer to q
    pub fn indep_ans_to_q(&mut self,a: &mut ans::Ansbox,qi:usize,qr:(i32,i32)) -> i32 {
        (*a).obj_ans(qr,self.db.ans[&qi].clone(),self.db.obj[&qi].clone())
    }

    /// # description
    /// instantiates a df::DPath used for node delegation for a question
    pub fn delegate(&mut self,qi:usize) {
        let mut dp = df::DPath{sm:HashMap::new(),na:HashMap::new(),
            head:self.idn,next_ref:Vec::new(),dscore: None};
        dp.sm.insert(self.idn,Vec::new());
        self.db.delegation_path = Some(dp);
        let mut db2 = self.db.clone();
        self.delegate_one(&mut db2,qi);
        self.db = db2;
    }

    /// # description
    /// fetch all neighbors that satisfy objective based on satisfaction
    /// rate map `sat_other`. 
    pub fn delegate_one(&mut self,db: &mut rndb::RNDB,qi:usize) {
        let mut dep = (*db).delegation_path.clone().unwrap();
        dep.sm.insert(self.idn,Vec::new());
        let neighs = self.neighbors.clone();        

        for n in neighs.iter() {
            let mut i:f32 = 0.;

            // case: neighbor of node already in delegation path search map
            if dep.sm.contains_key(n) {
                continue; 
            }

            let mut stat:bool = false;
            for x in dep.next_ref.iter() {
                if *x == *n {
                    stat = true;
                    break;
                }
            }

            // case: neighbor of node already a reference
            if stat {
                continue;
            }

            // fetch satisfaction rate 
            let i = self.fetch_neighbor_qsat_rate(db,*n,qi);

            if i >= 0.5 {
                // case: qualifying neighbors
                let mut x = dep.sm.get_mut(&self.idn).unwrap();
                x.push(*n);

                // add to next ref
                dep.next_ref.push(*n);
            }
        }
        (*db).delegation_path = Some(dep);
    }

    /// # description
    /// fetches the query satisfication rate of other node `ni` for question `qi`.
    /// 
    /// # NOTE
    /// misnomer for function: not just neighbors but any other node. 
    pub fn fetch_neighbor_qsat_rate(&mut self,db: &mut rndb::RNDB,ni:usize,qi:usize) -> f32 {        
        let mut qsat:f32 = 0.;

        // case: neighbor is registered in sat other  
        if (*db).sat_other.contains_key(&ni) {

            // case: question never delegated to neighbor
            if !(*db).sat_other[&ni].contains_key(&qi) {
                (*db).sat_other.get_mut(&ni).unwrap().insert(qi,1.);
            } 
        } else {
            // make new map
            let mut m2 : HashMap<usize,f32> = HashMap::new();
            m2.insert(qi,1.);
            (*db).sat_other.insert(ni,m2);
        }

        (*db).sat_other[&ni][&qi]
    }

    /// # description
    /// determines whether to delegate question `qi`. 
    /// # return 
    /// True -> delegate
    pub fn choose_to_delegate(&mut self,qi:usize) -> bool {
        // get rfeedback score
        let mut r:f32 = 0.;
        if self.db.rfeedback.contains_key(&qi) {
            let l = self.db.rfeedback[&qi].len();
            r = self.db.rfeedback[&qi][l - 1];
        } else {
            self.db.rfeedback.insert(qi,vec![0.]);
            r = 0.;
        }

        // compare with delegation score
        !(r < self.db.delegation_path.as_ref().unwrap().dscore.unwrap())
    }

    /// # description 
    /// processes node delegation; outputs the mean answer of the delegate nodes
    pub fn process_delegation(&mut self,qi:usize,ans_range:(i32,i32),node_ans:i32) -> Option<i32> {
        assert!(!self.db.delegation_path.is_none());
        let na = self.db.delegation_path.as_ref().unwrap().na.clone();

        if na.len() == 0 {
            return None; 
        }

        let mut a: i32 = 0;
        let ard = ans_range.1 - ans_range.0;
        for (k,v) in na.into_iter() {
            let dx = (node_ans - v).abs() as f32 / (ard as f32); 
            self.mod_delegation_record(qi,k,dx);
            a += v;
        }

        let a_: f32 = (a as f32) / ((1 + 
            self.db.delegation_path.as_ref().unwrap().na.len()) as f32);
        Some(a_.round() as i32)
    }

    pub fn mod_delegation_record(&mut self,qi:usize,ni:usize,s:f32) {
        // case: no question key 
        if !self.db.delegation_records.d1.contains_key(&qi) {
            self.db.delegation_records.d1.insert(qi,HashMap::new());
            self.db.delegation_records.d2.insert(qi,HashMap::new());
        }

        // case: no node key for question
        if !self.db.delegation_records.d1[&qi].contains_key(&ni) {
            self.db.delegation_records.d1.get_mut(&qi).unwrap().insert(ni,0.);
            self.db.delegation_records.d2.get_mut(&qi).unwrap().insert(ni,0);
        }

        // calculate new average
        let x1 = self.db.delegation_records.d1[&qi][&ni];
        let x2 = self.db.delegation_records.d2[&qi][&ni];
        let v = (x1 + s) / (x2 + 1) as f32;

        // update d1 and d2
        self.db.delegation_records.d1.get_mut(&qi).unwrap().insert(ni,v);
        self.db.delegation_records.d2.get_mut(&qi).unwrap().insert(ni,x2 + 1);
    }

    /// # description
    /// updates db sat_other map 
    pub fn update_sat_map(&mut self,qi:usize,ans_range:(i32,i32),node_ans:i32,c:f32) -> Option<i32> {
        let del = self.process_delegation(qi,ans_range,node_ans);
        let na = self.db.delegation_path.as_ref().unwrap().na.clone();
        for (k,v) in na.into_iter() {
            let s = 1. - self.db.delegation_records.d1[&qi][&k] * c * 
                    self.db.delegation_records.d2[&qi][&k] as f32;
            self.db.sat_other.get_mut(&k).unwrap().insert(qi,s);
        }
        del
    }

    /// # description
    /// deletes `qi` from `neighbors` attribute; used in the
    /// case when `qi` is a dead node.  
    pub fn delete_neighbor(&mut self, qi:usize) {
        let n:Vec<usize> = self.neighbors.clone().into_iter().filter(|x| *x != qi).collect();
        self.neighbors = n;
    }
}