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

mod qsbf; 
mod f1pattern;
use ndarray::{arr1,arr2,Array1,Array2,Dim,s};

fn main() {
    ////
    
    let mut r = rnb_env::sample_RNBENV1();
    rnb_env::run_rnb(&mut r);
     
    ////
    /*
    let mut i = cng::std_random_IRFDNG((-22,515));
    for j in 0..1000 {
        println!("{}: {}",j,i.next());
    }
    */ 
    ////
    /*
    let x = q_struct::arr1_index_to_arr2_index(28,(11,5));
    println!("{:?}",x); 
    */
}