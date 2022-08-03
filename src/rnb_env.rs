use crate::q_struct;
use crate::rnetwork;
use crate::rnode;
use std::collections::{HashMap,HashSet};

pub struct RNBENV {
    q: q_struct::QStruct,
    rn: rnetwork::RNetwork
}

pub fn build_RNBENV(q:q_struct::QStruct,rn: rnetwork::RNetwork) -> RNBENV {
    RNBENV{q:q,rn:rn} 
}

impl RNBENV {

    // transmits query from Q to a 
    pub fn prompt_node(&mut self) {
        // let qi = prompt Q;
        // let ni = prompt Q;

        // prompt node 
    }

    pub fn execute_query_on_node(&mut self,ni:usize,qi:usize,verbose:bool) {
        // have node perform delegation trial
        self.node_delegation(ni,qi,verbose);
        let eni = self.rn.node_idn_to_index(ni);

        // clone node db
        let db2 = self.rn.nodes[eni].db.clone();

        //// calculate dscore (resistance) for delegation

        // fetch node ans
        let qr = self.q.qs[qi].ans_range.clone();
        let o = self.rn.nodes[eni].db.obj[&qi].clone();
        let ka = self.rn.nodes[eni].db.ans[&qi].clone();
        let mut na = self.rn.ans_box.obj_ans(qr.clone(),ka,o);

        // fetch delegation ans 
        let da = self.rn.nodes[eni].update_sat_map(qi,qr,na,self.rn.c);
        if verbose {
            println!("delegation answer: {}",da);
        }

        // fetch Q ans
        let qa = self.q.ans_to_q(qi); 
        if verbose {
            println!("Q answer: {}",qa);
        }

        let rd = (qa - da).abs() as f32;        
        let mut dp = self.rn.nodes[eni].db.delegation_path.clone().unwrap();
        dp.dscore = Some(rd);
        self.rn.nodes[eni].db.delegation_path = Some(dp);

        // let node decide
        let node_del = self.rn.nodes[eni].choose_to_delegate(qi);
        if verbose {
            println!("node {} will delegate: {}",ni,node_del);
        }

        // let Q respond
        let mut nidns:HashSet<usize> = HashSet::new();
            // case: delegation 
        if node_del {
            nidns = HashSet::from_iter(
                self.rn.nodes[eni].db.delegation_path.as_ref().unwrap().na.clone().into_keys());
            na = da;
        } else {
            // case: no delegation, restore node db
            self.rn.nodes[eni].db = db2;
        }

        nidns.insert(ni);
        self.execute_Q_response_to_nodeset(ni,nidns,qi,na);

        // update node resistance
        let rd = (qa - na).abs() as f32;
        self.rn.nodes[eni].resistance -= rd; 

        if verbose {
            println!("node {} answer: {}",ni,na); 
        }

        if verbose {
            println!("resistance: {}",rd);
        }

        // update node resistance map
            // case: node delegated
        if node_del {
            return; 
        }

        if self.rn.nodes[eni].db.rfeedback.contains_key(&qi) {
            self.rn.nodes[eni].db.rfeedback.get_mut(&qi).unwrap().push(rd);
        }
    }

    pub fn execute_Q_response_to_nodeset(&mut self,srcidn:usize,node_set:HashSet<usize>,qi:usize,nodeset_ans:i32) {
        // have q respond to nodeset answer
        self.q.response_to_nodeset(srcidn,node_set,qi,nodeset_ans);
    }
    
    //////////////////// node delegation functions 

    pub fn node_delegation(&mut self,ni:usize,qi:usize,verbose:bool) {
        self.node_delegation_on_query(ni,qi,verbose);
        self.prompt_node_delegate_answers(ni,qi,verbose);
    }

    pub fn node_delegation_on_query(&mut self,ni:usize,qi:usize,verbose:bool) {
        if verbose {
            println!("delegate travel for src {} question {}",ni,qi);
        }
        
        // instantiate delegation
        let eni = self.rn.node_idn_to_index(ni);
        self.rn.nodes[eni].delegate(qi);

        // delegate
        let mut x = self.rn.nodes[eni].db.clone();
        let mut l = x.delegation_path.as_ref().unwrap().next_ref.len();
        while l > 0 {
            // clear next ref
            let r = x.delegation_path.as_ref().unwrap().next_ref.clone();
            
            if verbose {
                println!("delegation references: {:?}",r);
            }

            let mut dp = x.delegation_path.clone().unwrap();
            dp.next_ref = Vec::new();
            x.delegation_path = Some(dp);

            // collect values for next ref 
            for r_ in r.into_iter() {
                let ni2 = self.rn.node_idn_to_index(r_);
                self.rn.nodes[ni2].delegate_one(&mut x,qi);
            }
            l = x.delegation_path.as_ref().unwrap().next_ref.len();
        }

        self.rn.nodes[eni].db = x;
    }
    
    pub fn prompt_node_delegate_answers(&mut self,si:usize,qi:usize,verbose:bool) {
        let mut c: HashSet<usize> = HashSet::new();
        
        c.insert(si);
        if verbose {
            println!("delegate answers for src {} question {}",si,qi);
        }

        // get delegation path of source
        let esi0 = self.rn.node_idn_to_index(si);
        let mut dp = self.rn.nodes[esi0].db.delegation_path.clone().unwrap();
        let mut q: Vec<usize> = vec![dp.head];
        let mut l = q.len();
        while l > 0 {
            // pop element 0
            let e0 = q[0];
            let esi = self.rn.node_idn_to_index(e0);
            q = q[1..].to_vec();

            // get q rnange
            let qr = self.q.qs[qi].ans_range.clone();

            // get node ans
            let ka = self.rn.nodes[esi].db.ans[&qi].clone();

            // get node objective 
            let o = self.rn.nodes[esi].db.obj[&qi].clone();

            // add node answer if node not head
            if e0 != dp.head {
                // let node answer
                let ans = self.rn.ans_box.obj_ans(qr,ka,o);
                dp.na.insert(e0,ans);

                if verbose {
                    println!("node {} answer {}",e0,ans);
                }
            }

            // add its qualifying neighbors to cache
            let neigh = self.rn.nodes[esi].neighbors.clone();
            for ne in neigh.into_iter() {
                if c.contains(&ne) {
                    continue;
                }

                q.push(ne);
                c.insert(ne);
            }
            l = q.len();
        }

        self.rn.nodes[esi0].db.delegation_path = Some(dp);
    }

    pub fn fetch_node(&mut self,ni:usize) -> &rnode::RNBNode {
        let eni = self.rn.node_idn_to_index(ni);
        &self.rn.nodes[eni]
    }

    pub fn fetch_QStruct(&mut self) -> &mut q_struct::QStruct {
        &mut self.q
    }

}

pub fn sample_RNBENV1() -> RNBENV {
    let q = q_struct::sample_QStruct1();
    let r = rnetwork::sample_RNBNetwork1();
    build_RNBENV(q,r)
}