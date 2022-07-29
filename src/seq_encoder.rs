use ndarray::Array1;
use ndarray::arr1;
use ndarray::Dim;
use std::fmt;

/*
element of IndexFractionNotation
*/
#[derive(Clone)]
pub struct FloorDiv {
    pub t: Option<i32>,
    pub b: i32,
    pub m: i32
}

impl FloorDiv {

    /*
    if t is not none, t/b * m
    else, i/b * m
    */
    pub fn value(&mut self, i: i32) ->i32 {
        if self.t.is_none() {
            let x: f64 = (i as f64) / (self.b as f64);
            return (x.floor() as i32) * self.m;// as i32;
        }

        let mut q = self.t.unwrap();
        let q2:f64 = f64::from(i32::from(q));
        let q3:f64 = q2 / (self.b as f64);
        (q3 as i32) * self.m
    }

    pub fn clone(&mut self) -> FloorDiv {
        FloorDiv{t:self.t.clone(),b:self.b,m:self.m}
    }

}

impl fmt::Display for FloorDiv {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut q = &format!("* floor div: {:?} | {} | {} ",self.t,self.b,self.m);
        write!(f, "{}", q)
    }
}

/*
A sequence encoder that encodes all values of an Array1<i32>
by a Vec<FloorDiv> of equal length.
*/
pub struct IndexFractionNotation {
    pub v: Array1<i32>,
    pub divs: Vec<FloorDiv>
}

pub fn build_index_fraction_notation(v_: Array1<i32>) -> IndexFractionNotation {
    IndexFractionNotation{v:v_,divs:Vec::new()}
}

impl IndexFractionNotation {

    pub fn process(&mut self) {
        for i in 0..self.v.len() {
            let i2:i32 = i.clone() as i32;
            let x = self.output(i2);
            let y = self.v[i] - x;

            let ft = self.get_floordiv(i as i32,y);
            if ft.is_none() {
                continue;
            }
            let ft2 = ft.unwrap();
            self.divs.push(ft2.clone());
        }
    }

    /*
    */
    pub fn get_floordiv(&mut self, i:i32,y:i32) -> Option<FloorDiv> {

        if i == 0 {
            return Some(FloorDiv{t:Some(y),b:1,m:1});
        }

        // case: no change
        if y == 0 {
            return None;
        }

        Some(FloorDiv{t:None,b:i,m:y})
    }

    /*
    outputs the value for
    */
    pub fn output(&mut self, i: i32) -> i32 {
        if self.divs.len() == 0 {
            return i;
        }
        let mut x: i32 = 0;
        for q in self.divs.iter() {
            let mut xx = if (*q).t.is_none() {01} else {(*q).t.unwrap()};
            let r:&mut FloorDiv = &mut ((*q).clone());

            let r2 = (*r).value(i);

            // TODO: terminate here if 0.
            x += r2;
        }
        x
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_FloorDiv_value() {
        let mut x = FloorDiv{t:Some(3),b:4,m:1};
        let mut x2 = FloorDiv{t:None,b:4,m:1};
        let mut x3 = FloorDiv{t:None,b:4,m:4};

        assert_eq!(0,x.value(8));
        assert_eq!(0,x.value(10));
        assert_eq!(0,x.value(1002));

        assert_eq!(2,x2.value(8));
        assert_eq!(2,x2.value(10));
        assert_eq!(250,x2.value(1002));

        assert_eq!(1000,x3.value(1002));
    }

    #[test]
    fn test_IndexFractionNotation_output() {
        // case 1
        let q = arr1(&[0,5,6,8,11,14]);
        let mut ifn = build_index_fraction_notation(q.clone());
        ifn.process();
        for i in 0..6 {
            let mut j = ifn.output(i);
            let mut g = q[i as usize];
            assert_eq!(j,g);
        }

        // case 2
        let q2 = arr1(&[-3,14,7,10,18]);
        let ans2 = arr1(&[-3,14,7,10,18,35,14]);
        let mut ifn2 = build_index_fraction_notation(q2.clone());
        ifn2.process();
        for i in 0..7 {
            let j = ifn2.output(i);
            let  g = ans2[i as usize];
            assert_eq!(j,g);
        }

        // case 3
        let q3 = arr1(&[5,5,5,5,5]);
        let ans3 = arr1(&[5,5,5,5,5,5,5]);
        let mut ifn3 = build_index_fraction_notation(q3.clone());
        ifn3.process();
        for i in 0..7 {
            let mut j = ifn3.output(i);
            let mut g = ans3[i as usize];
            assert_eq!(j,g);
        }
    }
}
