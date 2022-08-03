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

use ndarray::{arr1,arr2,Array1,Array2,Dim,s};

fn main() {
    // test out RNDB after calling node delegation
    let mut r = rnb_env::sample_RNBENV1();
    r.execute_query_on_node(0,0,true);
}