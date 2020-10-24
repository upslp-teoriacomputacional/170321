## Datos personales

 - Universidad Politécnica de San Luis Potosí
 - Benjamín Loredo Amaya
 - 170321
 - Ingeniería en Tecnologías de la información
 - Teoría Computacional
 - Juan Carlos González Ibarra
 
## Objetivos
1. Realizar un programa que simule el funcionamiento de un compilador

## Como solucionaste el problema
Utilicé un HasSet para almacenar las palabras.

## Código

    use std::collections::HashSet;

    fn main() {

    let mut tokens = HashSet::new(); // for string tokens
    //let source_code: string = "int result = 1;".split() 
    // Loop through each source code word
    for word in "int result = 1;".split(" "){ // turning source code into list of words

        // This will check if a token has datatype decleration
        if (*word) == *"str" || *word == *"int" || *word == *"bool" {
            tokens.insert(["DATATYPE", word]);
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


## Problemas al programar
La sentencia if que verifica si el caracter es ; resultó en algo un tanto largo.

    if word.chars().nth(word.len()- 1 as usize).unwrap() == ';'

Para introducir una cadena al HashSet, se necesita albergarla en una variable que cumpla con ciertos requisitos con respecto al tiempo. Esta fue una de las soluciones que puse

      let y= ";";
      tokens.insert(["END_STATEMENT", &y]);
