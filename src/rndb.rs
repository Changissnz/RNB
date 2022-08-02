/*
respondent node database
*/
use std::collections::HashMap;
use crate::df;

#[derive(Clone)]
pub struct RNDB {
    // actual answers of node
    pub ans: HashMap<usize,Option<i32>>,

    // objective for each question
    // question id -> 0|1|2
    pub obj: HashMap<usize,usize>,

    // satisfaction rate of other nodes to self
    // node -> (question -> satisfaction rate)
    pub sat_other: HashMap<usize,HashMap<usize,f32>>,

    // current path used for delegation
    pub delegation_path: Option<df::DPath>,

    // question -> (delegate node -> contradiction)
    pub delegation_records: df::DelegationRecord,

    // resistance feedback for direct answer 
    // question -> resistance delta vec
    pub rfeedback: HashMap<usize,Vec<f32>>
}

/*
*/ 
pub fn build_RNDB(ans: HashMap<usize,Option<i32>>,obj: HashMap<usize,usize>) -> RNDB {
    RNDB{ans:ans,obj:obj,sat_other:HashMap::new(),delegation_path:None,
        delegation_records: df::DelegationRecord{d1:HashMap::new(),d2:HashMap::new()},rfeedback:HashMap::new()}
}

impl RNDB {

    // outputs an answer: no delegation
    pub fn output_ans(&mut self) -> i32 {
        1
    }
}