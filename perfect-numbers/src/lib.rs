#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    //unimplemented!("classify {}", num);
    
   
    let mut hasil =0;
    
    for i in 1..=(num/2){
        if num % i == 0{
            hasil += i
        }
    }

    
    if hasil > num {
        Some(Classification::Abundant)
    }
    else if hasil < num {
       Some(Classification::Deficient)
    }
    else {
       Some(Classification::Perfect)
    }
}
