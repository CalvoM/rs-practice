fn main() {
    let mut s = String::from("foo ");
    s.push_str("bar");
    s.push('.');
    println!("{}",s);
    let s1 = String::from("Five ");
    let s2 = String::from("Stones");
    let s3 = s1 + &s2;
    println!("{}", s3);
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // let game = tic + "-" + &tac + "-" + &toe;
    // println!("{}", game);
    let g = format!("{}-{}-{}",tic,tac,toe);
    println!("{}",g);
}
