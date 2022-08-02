/*
file contains delegator functions
*/ 
use std::collections::HashMap;
use std::fmt;

/*
delegation path: tree structure represented as a hashmap.
                 travel starts at key head. 
*/
#[derive(Clone)]
pub struct DPath {
    // search map: node to qualifying neighbors
    pub sm: HashMap<usize,Vec<usize>>,

    // node answers
    pub na: HashMap<usize,i32>,

    // map head
    pub head:usize,
    pub next_ref:Vec<usize>,
    pub dscore: Option<f32>

}

impl fmt::Display for DPath {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s1 = format!("search map\n{:?}\n",self.sm);
        let mut s2 = format!("node answers\n{:?}\n",self.na);
        let mut s3 = format!("head\n{}\n",self.head);
        let mut s4 = format!("next ref\n{:?}\n",self.next_ref);

        let mut q = "".to_string();
        q.push_str(&s1);
        q.push_str(&s2);
        q.push_str(&s3);
        q.push_str(&s4);
        write!(f, "{}", q)
    }
}

#[derive(Clone)]
pub struct DelegationRecord {
    // question -> (node -> average contradiction ratio)
    pub d1: HashMap<usize,HashMap<usize,f32>>,
    // question -> (node -> frequency of delegating question to node)
    pub d2: HashMap<usize,HashMap<usize,usize>>
}

