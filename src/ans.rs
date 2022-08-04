use crate::std_rng;
use crate::cng;


pub fn i32_in_range(r: (i32,i32),i:i32) -> bool {
    i >= r.0 && i <= r.1
}

/*
calculates a single value for i32 
*/ 
pub fn calculate_ans(ansrange:(i32,i32),known_ans:i32,dec_degree:f32) -> i32 {

    assert!(i32_in_range(ansrange,known_ans));

    if dec_degree == 0.0 {
        return known_ans;
    }

    let dvec = vec![known_ans - ansrange.0.clone(),ansrange.1.clone() - known_ans];
    let d = dvec.iter().max().unwrap();
    let dx = (dec_degree * *d as f32).round() as i32;
    
    // try adding and subtracting dx
    let mut s: Vec<i32> = Vec::new();
    if i32_in_range(ansrange,known_ans - dx) {
        s.push(known_ans - dx);
    }

    if i32_in_range(ansrange,known_ans + dx) {
        s.push(known_ans - dx);
    }

    let i = std_rng::random_i32_in_range((0,s.len() as i32 -1));
    s[i as usize]
}

pub fn invert_calculate_ans(ansrange:(i32,i32),known_ans:i32,actual_ans:i32) -> f32 {
    assert!(i32_in_range(ansrange,known_ans));
    assert!(i32_in_range(ansrange,actual_ans));

    if ansrange.1 - ansrange.0 == 0 {
        return 0.;
    }

    let dvec = vec![known_ans - ansrange.0.clone(),ansrange.1.clone() - known_ans];
    let d = dvec.iter().max().unwrap();
    ((known_ans - actual_ans).abs() as f32) / *d as f32
}

/*
Answer box; struct is used by all nodes in an RNBNetwork to answer
the questions of a QStruct.

For an RNBNode with its objective o in [0,1,2], and its known answer x for q, 
the AnsBox will provide RNBNode with an answer that it then uses to respond
back to the QStruct.

For objective 2, uses functions in std_rng
*/ 
pub struct Ansbox {
}

impl Ansbox {

    /// outputs a value in ansrange based on known answer and objective
    pub fn obj_ans(&mut self, ansrange:(i32,i32),known_ans:Option<i32>,obj:usize) -> i32 {
        let mut ka:i32 = 0;
        if !known_ans.is_none() {
            ka = known_ans.unwrap();
        } else {
            ka = ansrange.0 + ((ansrange.1 - ansrange.0) as f32 / 2.0).round() as i32;
        }

        // no deception
        let mut i:f32 = 0.;
        
        // deception
        if obj == 1 {
            i = 1.;
        }

        if obj == 2 {
            i = std_rng::random_f32_in_range((0.5,1.));
        }

        calculate_ans(ansrange,ka,i)
    }
}

