pub struct Triangle{
    one:u64,
    two:u64,
    three:u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        //unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        if (sides == [0,0,0]) || (sides[0] + sides[1] < sides[2])||(sides[0] + sides[2] < sides[1])||(sides[1] + sides[2] < sides[0]){
            None
        }
        
        else{
            let segitiga = Triangle {one:sides[0],two:sides[1],three:sides[2]};
            Some(segitiga)
            
        }
    }

    pub fn is_equilateral(&self) -> bool {
        //unimplemented!("Determine if the Triangle is equilateral.");
        return self.one==self.two && self.two==self.three
    }

    pub fn is_scalene(&self) -> bool {
        //unimplemented!("Determine if the Triangle is scalene.");
        return self.one!=self.two && self.one!=self.three && self.two!=self.three
    }

    pub fn is_isosceles(&self) -> bool {
        //unimplemented!("Determine if the Triangle is isosceles.");
        if self.one == self.two{
            return true
        }
        else if self.one == self.three {
            return true
        }
        else if self.two == self.three{
            return true
        }
        else {
            return false
        }
    }
}
