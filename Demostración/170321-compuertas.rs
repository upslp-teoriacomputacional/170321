/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    Benjamín Loredo Amaya
 *  Matricula: 170321
 *  
 *  Escrito:       20/09/2020
 *  Ultima actualización:  20/09/2020
 *----------------------------------------------------------------------*/


fn main(){

    
    
    //Comenzamos creando una lista con los dos posibles valores booleanos, False y True, 
    // que utilizaremos para iterar sobre ellos:
    let booleanos: [bool;2] =[false,true];
    
        
    //Muestra los conjuntos
    //println!("Conjunto de A: {:?}", conjuntoA);
    //Observa que no hemos rodeado los elementos entre comillas, pues no son strings.
    //A continuación imprimimos los títulos para la operación or:
    
    //Tabla de verdad de or
    
    // Los \t en la primera línea no son más que tabuladores. 
    //La segunda línea muestra 22 veces un carácter. Se coloca el carácter a imprimir después de los dos puntos
    //Seguido de un <,> o ^, según la posición donde queremos ubircar la cadena de caracteres. 
    //El parametro del final es una cadena que puede ir antes, en medio o después de los caracteres (<,> o ^). 
    println!("x\ty\tx or y");
    println!("{:-<22}", ""); 
    
    // El quid de la cuestión recae en la doble iteración usando el bucle for: 
    //Se utilizan * porque el operador OR necesita una variable x no &x
    for x in &booleanos {
        for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x || *y);
        }
    }
    
   //Tabla de verdad de and
    println!("\nx\ty\tx and y");
    println!("{:-<22}", ""); 
    for x in &booleanos {
        for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x && *y);
        }
    }
    
    //Tabla de verdad de not
    println!("\nx\tnot x");
    println!("{:-<13}", ""); 
    for x in &booleanos {
        println!("{:?}\t{:?}", x,!x);
        
    }
   
   //Tabla de verdad de ^
   println!("\nx\ty\tx ^ y");
    println!("{:-<22}", ""); 
    for x in &booleanos {
        for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x ^ *y);
        }
    }

}