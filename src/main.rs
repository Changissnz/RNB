mod seq_encoder;
mod std_rng;
mod cng;
mod rnode;
mod q_struct;
mod rdata;
mod rndb;
mod df;
mod rnetwork;
mod rnb_env;
mod ans;
mod rnb_env_demo;

fn main() {
        
    // case: n = 0, q = 0
    let mut rn = rnb_env::sample_RNBENV1();
    rn.node_delegation_on_query(0,0,true);
    rn.prompt_node_delegate_answers(0,0,true);    
}