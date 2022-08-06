use ndarray::{arr1,arr2,Array,Array1,Array2,s};
/*
QStruct bot functions
*/ 

/// function that determines one node to fix by fix F2
/// *F2* place restriction on node so it can no longer be a delegate
///        to any other node. 
/// uses the following chain:
/// (1) calculate subset s of nodes that were delegates to any other node
/// (2) get MIN (n in s) [delegation_count(n) * |ans(Q) - mean_ans(n)|]
///  
/// NOTE: chain-function is "static", outputting
///       values based on the procedure described above
///       and will not calculate the best decision QStruct
///       can make in specific cases of RNBNetwork. 
/// return: (node identifier,score) 
pub fn qbot_function_1(z:Array2<i32>,w:Array2<usize>,wanted_answers:Array1<i32>) -> Option<(usize,i32)> {
    
    let q:Vec<usize> = delegate_nodes(w.clone());
    if q.len() == 0 {return None;}

    let mut scores:Vec<i32> = Vec::new();
    for i in q.clone().into_iter() {
        let mut z1:Array1<i32> = z.slice(s![i,..]).to_owned();
        let mut w1:Array1<i32> = w.slice(s![i,..]).to_owned().into_iter().map(|x| x as i32).collect();
        //println!("node {} score {}",i,s1);
        scores.push(qbot_base_function(z1,w1,wanted_answers.clone())); 
    }

    let n = scores.into_iter().enumerate().fold((0,i32::MAX),|x1,x2| if x1.1 <= x2.1 {x1} else {x2});
    Some((q[n.0],n.1))
}

/// calculates the delta of QStruct.c (struct's fuel) if F2 is applied to node
/// n with mean answers z to questions, delegation count w, and QStruct's wanted
/// values `wanted_answers`. 
pub fn qbot_base_function(z:Array1<i32>,w:Array1<i32>,wanted_answers:Array1<i32>) -> i32 {
    (w * (z - wanted_answers.clone())).into_iter().map(|x| x.abs()).sum()
}

/// calculates subset of nodes that were delegates to any other node
pub fn delegate_nodes(w: Array2<usize>) -> Vec<usize> {
    let (r,_) = w.dim();

    let mut qi: Vec<usize> = Vec::new(); 
    // collect all questions with  >= 1 nodes that did not answer
    for i in 0..r {
        let r2:Array1<usize> = w.slice(s![i,..]).to_owned();
        let r3:Array1<usize> = r2.into_iter().filter(|x| *x > 0).collect(); 
        if r3.len() > 0 {
            qi.push(i);
        }
    }
    qi 
}

pub fn qbot_function_1__test_case1() -> (Array2<i32>,Array2<usize>,Array1<i32>) {

    let mut z:Array2<i32> = arr2(&[[3,2,4,5,1,0],
        [1,1,1,5,7,-3],
        [3,2,3,5,6,-3],
        [2,2,3,5,0,4],
        [3,2,4,5,1,4]]);

    let mut w:Array2<usize> = arr2(&[[3,2,4,5,1,0],
                    [1,1,1,0,0,0],
                    [0,0,0,0,0,0],
                    [2,0,2,1,0,0],
                    [0,0,0,0,0,0]]);
    
    let mut wa: Array1<i32> = arr1(&[3,2,2,3,-1,0]);
    (z,w,wa)
}

pub fn qbot_function_1__test_case2() -> (Array2<i32>,Array2<usize>,Array1<i32>) {

    let mut z:Array2<i32> = arr2(&[[3,2,4,5,1,0],
        [1,1,1,5,7,-3],
        [3,2,2,3,-1,0],
        [2,2,3,5,0,4],
        [3,2,4,5,1,4]]);

    let mut w:Array2<usize> = arr2(&[[3,2,4,5,1,0],
                    [1,1,1,0,0,0],
                    [3,2,4,5,1,0],
                    [2,0,2,1,0,0],
                    [0,0,0,0,0,0]]);
    
    let mut wa: Array1<i32> = arr1(&[3,2,2,3,-1,0]);
    (z,w,wa)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__qbot_function_1() {
        // case 1
        let (z,w,wa) = qbot_function_1__test_case1();
        let x = qbot_function_1(z,w,wa);
        assert_eq!(x.unwrap().0,1);

        // case 2
        let (z,w,wa) = qbot_function_1__test_case2();
        let x = qbot_function_1(z,w,wa);
        assert_eq!(x.unwrap().0,2);
    }
}