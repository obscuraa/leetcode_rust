//1342 Number of Steps to Reduce a Number to Zero  

pub fn number_of_steps (num: i32) -> i32 {   
     struct Counter {
        n: i32,
     }

    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<i32> {

            if self.n % 2 == 0 && self.n != 0 {
                self.n /= 2
            } else {
                self.n -= 1
            };

            if self.n >= 0 {
                Some(self.n)
            } else {
                None
            }
        }
    }

Counter { n: num }.count() as i32       
}

pub fn number_of_steps_slow (num: i32) -> i32 { 
    let mut counter = 0 as i32;
        
        while num != 0 {
            if num % 2 == 0 {
                num / 2;
            } else {
                num - 1;
                counter += 1;
            counter += 1;
            }
        }
        return counter;
}