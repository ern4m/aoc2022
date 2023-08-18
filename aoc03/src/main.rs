fn main() {
    let lower = 'a'..'z';
    let lower_prior = 1..26;
    let upper = 'A'..'Z';
    let upper_prior = 27..52;


    let p = lower.zip(lower_prior);
        
    dbg!(p);
}
