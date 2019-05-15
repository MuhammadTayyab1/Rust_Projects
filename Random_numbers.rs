use rand::Rng;
fn main() {
for i in 0..1000{
println!("Guess the number!");
let secret_number = rand::thread_rng().gen_range(1,
1000);
println!("{0}",secret_number);
  
}
}
