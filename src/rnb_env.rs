use crate::q_struct;
use crate::rnetwork;
use crate::rnode;
use crate::f1pattern;
use std::collections::{HashMap,HashSet};

pub struct RNBENV {
    q: q_struct::QStruct,
    rn: rnetwork::RNetwork
}

pub fn build_RNBENV(q:q_struct::QStruct,rn: rnetwork::RNetwork) -> RNBENV {
    RNBENV{q:q,rn:rn} 
}

impl RNBENV {

    /// performs summarization on nodes that can no longer
    /// resist and on Q
    /// return: if network is still active
    pub fn summarize_stats(&mut self,verbose:bool) -> bool {
        let mut stat = true; 
        if verbose {
            println!("Q fuel: {}",self.q.c);
            println!("Qdb");
            println!("{}",self.q.rd);
            println!("----");
        }
        if self.q.c <= 0 {
            stat = false; 
        }

        let l = self.rn.nodes.len();
        for i in 0..l {
            if verbose {
                println!("{}",self.rn.nodes[i]);
                println!("\t----"); 
            }
            if stat {
                stat = self.rn.nodes[i].resistance > 0.;
            } 
        }
        if verbose {
            println!("====");
        }

        stat 
    }

    /// executes one move by Q
    pub fn execute_Q_move(&mut self,verbose:bool) {
        let (i,i2) = self.q.one_move();

        if verbose {
            println!("executing query {} on node {}",i.1,i.0);
        }

        // fix by F2
        self.fix_F2(i2,verbose);

        // execute the query
        self.execute_query_on_node(i.0,i.1,verbose);

        // update QStruct fuel after executing query
        self.q.c -= 1; 
    }

    /// performs an F2 fix on node, node f.0 can no be
    /// be a delegate
    pub fn fix_F2(&mut self,f:Option<(usize,i32)>,verbose:bool) {
        if f.is_none() {
            return;
        }

        let (x1,x2) = f.unwrap();

        // mark node delegate status as false
        self.q.f2_nodes.insert(x1);

        if verbose {
            println!("\tnode {} is fixed by F2",x1);
        }

        // subtract f2 score from Q.c
        self.q.c -= x2; 

        if verbose {
            println!("\tQ fuel is: {}",self.q.c);
        }
    }

    /// iterates through nodes and apply F1 fix
    /// on them.
    pub fn fix_F1(&mut self) {
        let l = self.rn.nodes.len();

        // collect the ansrange vec 
        let qrvec:Vec<(i32,i32)> = self.q.qs.clone().into_iter().map(|x| x.ans_range).collect();
        for i in 0..l {
            // case: node with no resistance not fixed yet
            let stat = self.rn.nodes[i].f1.is_none() && self.rn.nodes[i].resistance <= 0.; 
            if stat {
                let mut f1 = rnode::default_F1_anspattern(&mut self.rn.nodes[i],&mut self.rn.ans_box,qrvec.clone());
                self.rn.nodes[i].f1 = Some(f1);
            }
        }
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
        //let o = self.rn.nodes[eni].db.obj[&qi].clone();
        //let ka = self.rn.nodes[eni].db.ans[&qi].clone();
        //let mut na = self.rn.ans_box.obj_ans(qr.clone(),ka,o);
        let mut na = self.rn.nodes[eni].ans_to_q(&mut self.rn.ans_box,qi,qr.clone());

        // fetch delegation ans 
        let da = self.rn.nodes[eni].update_sat_map(qi,qr,na,self.rn.c);
        if verbose {
            println!("delegation answer: {}",da);
            println!("DB after delegation:\n*************\n{}\n******************\n\n",self.rn.nodes[eni].db);
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
        self.rn.nodes[eni].resistance = self.rn.nodes[eni].resistance - rd; 

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

            // case: node e0 is in QStruct.f2_nodes
            if self.q.f2_nodes.contains(&e0) {
                l = q.len();
                continue;
            }

            // get q rnange
            let qr = self.q.qs[qi].ans_range.clone();

            // get node ans
            let ka = self.rn.nodes[esi].db.ans[&qi].clone();

            // get node objective 
            let o = self.rn.nodes[esi].db.obj[&qi].clone();

            // add node answer if node not head
            if e0 != dp.head {
                // let node answer
                let ans = self.rn.nodes[esi].ans_to_q(&mut self.rn.ans_box,
                    qi,qr.clone());
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

    pub fn fetch_node(&mut self,ni:usize) -> &mut rnode::RNBNode {
        let eni = self.rn.node_idn_to_index(ni);
        &mut self.rn.nodes[eni]
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

/// have Respondent Network Bot run until one of the following:
/// 1. all nodes in Respondent Network are fixed by F1.
/// 2. Q runs out of fuel. 
pub fn run_rnb(r: &mut RNBENV) {
    let mut stat:bool = (*r).summarize_stats(true);
    let mut c = 5; 
    while stat && c > 0 {
        (*r).execute_Q_move(true);
        stat = (*r).summarize_stats(true);
        c -= 1;
    }


}