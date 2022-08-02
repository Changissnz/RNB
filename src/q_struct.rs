use crate::rdata;
use crate::seq_encoder;
use crate::ans; 
use ndarray::{arr1,arr2,Array1,Array2,s};
use std::collections::HashSet;

/*
question struct
*/ 
pub struct Q {
    // answer to question 
    pub qa: Option<i32>,
    pub ans_range: (i32,i32)
}

pub struct QStruct {
    pub qs: Vec<Q>,
    pub rd: rdata::QData

}

pub fn build_QStruct(qs:Vec<Q>,r:usize) -> QStruct {
    let rd = rdata::build_QData(r,qs.len());
    QStruct{qs:qs,rd:rd}
}

impl QStruct {

    pub fn response_to_nodeset(&mut self,srcidn: usize, ns:HashSet<usize>,qi:usize,nodeset_ans:i32) {
        
        // log the response into data
        self.rd.log_node_response(srcidn,ns,qi,self.qs[qi].ans_range.clone(),self.qs[qi].qa.clone(),nodeset_ans);
    }

    /*
    method used in the case of known and unknown 
    */ 
    pub fn ans_to_q(&mut self,qi:usize) -> i32 {

        // case: QStruct has a known answer
        if !self.qs[qi].qa.is_none() {
            return self.qs[qi].qa.clone().unwrap();
        }

        // case: QStruct does not have known answer
        self.rd.average_ans_to_question(qi,self.qs[qi].ans_range.clone())
    }

    /*
    choose node to query with question
    */ 
    pub fn query_rnd(&mut self, ni:usize,qi:usize) {
        
    }

    /*
    queries a target node 
    */ 
    pub fn query_tgt(&mut self) {

    }

    /*
    sends a "fix node" message to network node 
    */ 
    pub fn fix_node_msg(&mut self) {// Network) {
    }
}

pub fn sample_QStruct1() -> QStruct {
    let q0 = Q{qa:Some(50),ans_range:(0,100)};
    let q1 = Q{qa:None,ans_range:(-80,80)};
    let q2 = Q{qa:Some(10),ans_range:(-50,25)};
    let q3 = Q{qa:Some(-25),ans_range:(-100,0)};
    let q4 = Q{qa:Some(6),ans_range:(0,10)};

    build_QStruct(vec![q0,q1,q2,q3,q4],11)
}