/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
    if s1.len() != s2.len(){
        None
    }
    else {
        let mut hasil = 0;
        let mut s1_char = s1.chars();
        let mut s2_char = s2.chars();
        for i in 0..s1.len(){
            if &s1_char.next()!= &s2_char.next(){
                hasil +=1;
            } 
        }
        Some(hasil)
    }
}
