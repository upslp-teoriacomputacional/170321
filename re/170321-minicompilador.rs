use std::collections::HashSet;

fn main() {

let mut tokens = HashSet::new(); // for string tokens
//let source_code: string = "int result = 1;".split() 
// Loop through each source code word
for word in "int result = 1;".split(" "){ // turning source code into list of words
    
    // This will check if a token has datatype decleration
    if (*word) == *"str" || *word == *"int" || *word == *"bool" {
        //tokens.insert(["DATATYPE", word])
    }
    // This will look for an identifier which would be just a word
    else {
      let mut aux: bool=true;
      for a in word.chars(){
        if !a.is_alphabetic(){
          aux=false;
        }
      }if aux {
        tokens.insert(["IDENTIFIER", word]);
      }
      else {
        // This will look for an operator
        if *word==*"-"||*word==*"/"||*word==*"+"||*word==*"*"||*word==*"%"||*word==*"="{
          tokens.insert(["OPERATOR", word]);
        }   
        // This will look for integer items and cast them as a number
        else{
          if word.chars().next().unwrap().is_digit(10){
            if word.chars().nth(word.len()- 1 as usize).unwrap() == ';'{
              //let x1 = word.chars().next().unwrap().to_string();
              let x="1";
              tokens.insert(["INTEGER",&x]);
              let y= ";";
              tokens.insert(["END_STATEMENT", &y]);
            } 
            else{
                tokens.insert(["INTEGER", word]);
            } 
          }
        } 
      }  
    }        
}
println!("{:?}",tokens); // Outputs the token array
}


