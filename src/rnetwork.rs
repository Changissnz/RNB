use crate::rnode;
use crate::df;
use crate::ans;
use crate::rndb;
use std::collections::HashMap;

pub struct RNetwork {
    pub nodes: Vec<rnode::RNBNode>,
    pub ans_box: ans::Ansbox,
    pub c: f32
}

pub fn build_RNetwork(m: HashMap<usize,Vec<usize>>,rndbvec:HashMap<usize,rndb::RNDB>,
        r:HashMap<usize,f32>,c:f32) -> RNetwork {
    let mut nodes:Vec<rnode::RNBNode> = Vec::new();
    for (k,v) in m.into_iter() {
        let rn = rnode::build_RNBNode(k,rndbvec[&k].clone(),v,r[&k].clone());
        nodes.push(rn);
    }
    RNetwork{nodes:nodes,ans_box:ans::Ansbox{},c:c} 
}

impl RNetwork {

    pub fn node_idn_to_index(&mut self,idn:usize) -> usize {
        let l = self.nodes.len();
        for i in 0..l {
            if self.nodes[i].idn == idn {
                return i;
            }
        }

        assert!(false);
        1
    }
}

pub fn sample_node_idn_map1() -> HashMap<usize,Vec<usize>> {
    let mut m: HashMap<usize,Vec<usize>> = HashMap::new();
    m.insert(0,vec![1,2,3,4,5]);
    m.insert(1,vec![0,7,9]);
    m.insert(2,vec![0,6,8,9]);
    m.insert(3,vec![0,7]);
    m.insert(4,vec![0,6]);
    m.insert(5,vec![0,6]);
    m.insert(6,vec![2,4,5]);
    m.insert(7,vec![1,3,9]);
    m.insert(8,vec![2,10]);
    m.insert(9,vec![1,2,7,10]);
    m.insert(10,vec![8,9]);
    m
}

/*
for use with sample_QStruct1.

"node x knows all answers" -> "node answers matches with those of Q"
*/ 
pub fn sample_rndb10() -> HashMap<usize,rndb::RNDB> {
    let mut v: Vec<rndb::RNDB> = Vec::new();

    // node 0 knows all answers, no deception.
    let v:Vec<(usize,Option<i32>)> = vec![(0,Some(50)),(1,Some(40)),(2,Some(10)),(3,Some(-25)),(4,Some(6))];
    let vh = HashMap::from_iter(v);
    let o:Vec<(usize,usize)> = vec![(0,0),(1,0),(2,0),(3,0),(4,0)];
    let obj = HashMap::from_iter(o);
    let mut rnb = rndb::build_RNDB(vh,obj);

    // node 1 does not know any answers, no deception.
    let v1 = vec![(0,None),(1,None),(2,None),(3,None),(4,None)];
    let vh1 = HashMap::from_iter(v1);
    let o1 = vec![(0,0),(1,0),(2,0),(3,0),(4,0)];
    let obj1 = HashMap::from_iter(o1);
    let mut rnb1 = rndb::build_RNDB(vh1,obj1);

    // node 2 knows all answers, all deception. 
    let v2 = vec![(0,Some(50)),(1,Some(-40)),(2,Some(10)),(3,Some(-25)),(4,Some(6))];
    let vh2 = HashMap::from_iter(v2);
    let o2 = vec![(0,1),(1,1),(2,1),(3,1),(4,1)];
    let obj2 = HashMap::from_iter(o2);
    let mut rnb2 = rndb::build_RNDB(vh2,obj2);

    // node 3 knows contradicting answers, all deception. 
    let v3 = vec![(0,Some(75)),(1,Some(60)),(2,Some(-40)),(3,Some(0)),(4,Some(2))];
    let vh3 = HashMap::from_iter(v3);
    let o3 = vec![(0,1),(1,1),(2,1),(3,1),(4,1)];
    let obj3 = HashMap::from_iter(o3);
    let mut rnb3 = rndb::build_RNDB(vh3,obj3);

    // node 4 knows contradicting answers equal to that of node 3, no deception. 
    let v4 = vec![(0,Some(75)),(1,Some(60)),(2,Some(-40)),(3,Some(0)),(4,Some(2))];
    let vh4 = HashMap::from_iter(v4);
    let o4 = vec![(0,0),(1,0),(2,0),(3,0),(4,0)];
    let obj4 = HashMap::from_iter(o4);
    let mut rnb4 = rndb::build_RNDB(vh4,obj4);

    // node 5 knows contradicting answers, no deception. 
    let v5 = vec![(0,Some(25)),(1,Some(70)),(2,Some(20)),(3,Some(-100)),(4,Some(3))];
    let vh5 = HashMap::from_iter(v5);
    let o5 = vec![(0,0),(1,0),(2,0),(3,0),(4,0)];
    let obj5 = HashMap::from_iter(o5);
    let mut rnb5 = rndb::build_RNDB(vh5,obj5);

    // node 6 knows all answers, contradiction.
    let v6 = vec![(0,Some(50)),(1,Some(40)),(2,Some(10)),(3,Some(-25)),(4,Some(6))];
    let vh6 = HashMap::from_iter(v6);
    let o6 = vec![(0,2),(1,2),(2,2),(3,2),(4,2)];
    let obj6 = HashMap::from_iter(o6);
    let mut rnb6 = rndb::build_RNDB(vh6,obj6);

    // node 7 knows answers 0,2,4, contradiction.
    let v7 = vec![(0,Some(50)),(1,Some(-20)),(2,Some(10)),(3,Some(0)),(4,Some(6))];
    let vh7 = HashMap::from_iter(v7);
    let o7 = vec![(0,2),(1,2),(2,2),(3,2),(4,2)];
    let obj7 = HashMap::from_iter(o7);
    let mut rnb7 = rndb::build_RNDB(vh7,obj7);

    // node 8 knows answers 2,4, all deception.
    let v8 = vec![(0,Some(95)),(1,Some(-25)),(2,Some(10)),(3,Some(0)),(4,Some(6))];
    let vh8 = HashMap::from_iter(v8);
    let o8 = vec![(0,1),(1,1),(2,1),(3,1),(4,1)];
    let obj8 = HashMap::from_iter(o8);
    let mut rnb8 = rndb::build_RNDB(vh8,obj8);

    // node 9 knows all answers, deception 0,1,2 and no deception 3,4
    let v9 = vec![(0,Some(50)),(1,Some(40)),(2,Some(10)),(3,Some(-25)),(4,Some(6))];
    let vh9 = HashMap::from_iter(v9);
    let o9 = vec![(0,1),(1,1),(2,1),(3,0),(4,0)];
    let obj9 = HashMap::from_iter(o9);
    let mut rnb9 = rndb::build_RNDB(vh9,obj9);
    
    // node 10 knows all answers, deception 3,4 and no deception 0,1,2 
    let v10 = vec![(0,Some(50)),(1,Some(40)),(2,Some(10)),(3,Some(-25)),(4,Some(6))];
    let vh10 = HashMap::from_iter(v10);
    let o10 = vec![(0,0),(1,0),(2,0),(3,1),(4,1)];
    let obj10 = HashMap::from_iter(o10);
    let mut rnb10 = rndb::build_RNDB(vh10,obj10);

    let rv = vec![(0,rnb),(1,rnb1),(2,rnb2),(3,rnb3),(4,rnb4),(5,rnb5),(6,rnb6),
        (7,rnb7),(8,rnb8),(9,rnb9),(10,rnb10)];
    HashMap::from_iter(rv)
}

pub fn sample_resistancevec1() -> HashMap<usize,f32> {
    let v = vec![(0,100.),(1,100.),(2,100.),(3,100.),(4,100.),(5,100.),(6,100.),
        (7,100.),(8,100.),(9,100.),(10,100.)];
    HashMap::from_iter(v)
}

pub fn sample_RNBNetwork1() -> RNetwork {
    build_RNetwork(sample_node_idn_map1(),sample_rndb10(),
    sample_resistancevec1(),1.)
}