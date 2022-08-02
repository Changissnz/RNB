use crate::rnb_env;
use crate::rnode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_RNBENV_execute_query_on_node() {
        // case: n0
        let mut r = rnb_env::sample_RNBENV1();
        r.execute_query_on_node(0,0,true);
        let mut n: &rnode::RNBNode = r.fetch_node(0);
        assert_eq!(100.,(*n).resistance); 

        // case: n2
        r = rnb_env::sample_RNBENV1();
        r.execute_query_on_node(2,0,true);
        n = r.fetch_node(2);
        assert_eq!(50.,(*n).resistance); 

    }


}