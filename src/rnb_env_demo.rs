use crate::rnb_env;
use crate::rnode;
use ndarray::{arr1,arr2,Array1,Array2,Dim,s};

/// RData soln to node 0, query 0
pub fn RData_soln__execute_query_on_node_00() -> (Array2<f32>,Array2<usize>,Array2<i32>) {
    let mut xsol:Array2<f32> = Array2::zeros((11,5));
    let mut ysol:Array2<usize> = Array2::zeros((11,5));
    let mut zsol:Array2<i32> = Array2::zeros((11,5));
    let d = Dim((0,0));
    ysol[d] = 1; 
    zsol[d] = 50; 
    (xsol,ysol,zsol)
}

/// RData soln to node 2, query 0
pub fn RData_soln__execute_query_on_node_20() -> (Array2<f32>,Array2<usize>,Array2<i32>) {
    let mut xsol:Array2<f32> = Array2::zeros((11,5));
    let mut ysol:Array2<usize> = Array2::zeros((11,5));
    let mut zsol:Array2<i32> = Array2::zeros((11,5));
    let d = Dim((2,0));
    xsol[d] = 1.; 
    ysol[d] = 1; 
    (xsol,ysol,zsol)
}

/// RData soln to node 1, query 0
pub fn RData_soln__execute_query_on_node_10() -> (Array2<f32>,Array2<usize>,Array2<i32>) {
    let mut xsol:Array2<f32> = Array2::zeros((11,5));
    let mut ysol:Array2<usize> = Array2::zeros((11,5));
    let mut zsol:Array2<i32> = Array2::zeros((11,5));
    let d = Dim((1,0));
    xsol[d] = 1.; 
    ysol[d] = 50; 
    (xsol,ysol,zsol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_RNBENV_execute_query_on_node__question0() {
        // case: n0
        let mut r = rnb_env::sample_RNBENV1();
        r.execute_query_on_node(0,0,false);
        let mut n: &rnode::RNBNode = r.fetch_node(0);
        assert_eq!(100.,(*n).resistance); 

        let (x,y,z) = RData_soln__execute_query_on_node_00();
        let mut q = r.fetch_QStruct();
        assert_eq!((*q).rd.x,x); 
        assert_eq!((*q).rd.y,y); 
        assert_eq!((*q).rd.z,z); 

        // case: n2
        r = rnb_env::sample_RNBENV1();
        r.execute_query_on_node(2,0,false);
        n = r.fetch_node(2);
        assert_eq!(50.,(*n).resistance); 

        let (x2,y2,z2) = RData_soln__execute_query_on_node_20();
        q = r.fetch_QStruct();
        assert_eq!((*q).rd.x,x2); 
        assert_eq!((*q).rd.y,y2); 
        assert_eq!((*q).rd.z,z2); 

        // case: n1
        r = rnb_env::sample_RNBENV1();
        r.execute_query_on_node(1,0,false);
        n = r.fetch_node(2);
        assert_eq!(100.,(*n).resistance); 
        q = r.fetch_QStruct();
        let (x3,y3,z3) = RData_soln__execute_query_on_node_10();
        assert_eq!((*q).rd.x,x3); 
        assert_eq!((*q).rd.y,y3); 
        assert_eq!((*q).rd.z,z3); 
    }

    #[test]
    fn test_RNBENV_execute_query_on_node__question1() {
        // case: n0
        let mut r = rnb_env::sample_RNBENV1();
        let mut q = r.fetch_QStruct();
        let d = Dim((0,1));
        assert_eq!(0,(*q).ans_to_q(1)); 
    
        r.execute_query_on_node(0,1,false);
        q = r.fetch_QStruct();
        assert_eq!(0.5,(*q).rd.x[d]);
        assert_eq!(1,(*q).rd.y[d]);
        assert_eq!(40,(*q).rd.z[d]);
        assert_eq!(40,(*q).ans_to_q(1)); 
    
        r.execute_query_on_node(0,1,false);
        q = r.fetch_QStruct();
        assert_eq!(0.25,(*q).rd.x[d]);
        assert_eq!(2,(*q).rd.y[d]);
        assert_eq!(40,(*q).rd.z[d]);
        assert_eq!(40,(*q).ans_to_q(1)); 
    
        r.execute_query_on_node(0,1,false);
        q = r.fetch_QStruct();
        assert_eq!(0.5 /3.,(*q).rd.x[d]);
        assert_eq!(3,(*q).rd.y[d]);
        assert_eq!(40,(*q).rd.z[d]);
        assert_eq!(40,(*q).ans_to_q(1)); 
    
        r.execute_query_on_node(0,1,false);
        q = r.fetch_QStruct();    
        assert_eq!(0.125,(*q).rd.x[d]);
        assert_eq!(4,(*q).rd.y[d]);
        assert_eq!(40,(*q).rd.z[d]);
        assert_eq!(40,(*q).ans_to_q(1)); 
    }


}