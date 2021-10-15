fn main() {
    let data = [("first", "A Partridge in a Pear Tree"),
   ("second", "Two Turtle Doves"),
   ("third", "Three French Hens"),
   ("fourth", "Four Calling Birds"),
   ("fifth", "Five Golden Rings"),
   ("sixth", "Six Geese a Laying"),
   ("seventh", "Seven Swans a Swimming"),
   ("eighth", "Eight Maids a Milking"),
   ("ninth", "Nine Ladies Dancing"),
   ("tenth","Ten Lords a Leaping"),
   ("eleventh", "Eleven Pipers Piping"),
   ("twelfth", "12 Drummers Drumming")];
   for d in data.iter(){
       println!("On the {} day of Christmas, my true love gave to me {}",d.0,d.1);
   }
}
