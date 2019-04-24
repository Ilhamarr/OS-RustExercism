use std::collections::HashSet;

//Saya Kerjakan Masalah ini dibantu oleh M. Luthfi Albiansyah

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);
    let mut hasil = HashSet::new();

    for a in 1..sum/3{
        for b in 1..sum/2{
            let c = sum -(a+b);

            if ((a*a)+(b*b)==(c*c)){
                if a<b && b<c {
                    hasil.insert([a,b,c]);
                }
                
            }
            
        }
            
    }   
    hasil
}
