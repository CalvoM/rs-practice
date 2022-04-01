pub struct Currency{
    pub name: String,
    pub value: f32,
}

pub fn currency_converter(src: &Currency, dest: &mut Currency, rate: f32){
    println!("Converting from {} to {} @{}", src.name, dest.name, rate);
    dest.value = src.value * rate;
}