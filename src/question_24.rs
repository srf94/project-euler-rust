fn factorial(val: usize) -> usize {
   if val > 1 { return val * factorial(val - 1); }
   1
}


fn main() {
   let number_digits = 10;
   let target = 999999;  // Note '0' is the 1st permutation

   let mut digits_to_pick = Vec::new();
   for i in 0..number_digits { digits_to_pick.push(i); }

   let mut digits = Vec::new();
   let mut num = target;
   for i in (2..number_digits).rev() {
       let fac = factorial(i);
       println!("{}", fac);
       let K = num / fac;
       println!("{}", K);
       num = num - K * fac;

       let picked_digit = digits_to_pick[K];
       digits.push(picked_digit);
       digits_to_pick.retain(|&x| x != picked_digit);

   }

   if target % 2 == 0 {
       println!("0");
       digits.push(digits_to_pick[0]);
       digits.push(digits_to_pick[1]);
   } else {
       println!("1");
       digits.push(digits_to_pick[1]);
       digits.push(digits_to_pick[0]);
   }

   println!("{:?}", digits);
}