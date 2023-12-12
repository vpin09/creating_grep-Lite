use regex::Regex;
fn main() {
   let re=Regex::new("picture").unwrap();
   let quote=" \
   Every face, every shop, bedroom window, public-house, and
   dark square is a picture feverishly turned--in search of what?
   It is the same with books. What do we seek through millions of pages?";
   // let mut line_num=1;
   for line in quote.lines() {
      let contain_substring=re.find(line);
      match contain_substring {
         Some(_) => println!("{}",line),
         None =>(),
      }
      
   }


}
