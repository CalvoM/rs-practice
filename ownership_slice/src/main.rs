fn main() {
    let name: String = String::from("Noel Mwadime");
    let end = first_word(&name);
    let first = &name[0..end];
    let second = &name[end+1..name.len()];
    println!("{} {} == {}",first,second,name);
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
