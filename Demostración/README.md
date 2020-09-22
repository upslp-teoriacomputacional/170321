## Datos personales

 - Universidad Politécnica de San Luis Potosí
 - Benjamín Loredo Amaya
 - 170321
 - Ingeniería en Tecnologías de la información
 - Teoría Computacional
 - Juan Carlos González Ibarra
 
## Objetivos
1. Abordar lo esencial sobre las operaciones lógicas en el lenguaje Rust
### Operaciones utilizadas
AND OR  NOT XOR

##Como solucionaste el problema
En el lenguaje Rust existen los operadores lógicos. 
AND = &&
OR = ||
NOT = !
XOR =  ^

##Explicación del código

    let booleanos: [bool;2] =[false,true];
Se define un arreglo con dos booleanos utilizando la sintaxis de Rust

###Tabla de AND

    println!("x\ty\tx or y");
    println!("{:-<22}", ""); 
    for x in &booleanos {
        for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x || *y);
        }
    }

Primero se imprimen los títulos de la tabla. En la segunda línea se utiliza un "truco" de Rust que permite imprimir un carácter varias veces.
Se uilizan dos ciclos for anidados para recorrer los valores del arreglo en sus distintas combinaciones.
###Tabla de OR

    println!("\nx\ty\tx and y");
    println!("{:-<22}", ""); 
    for x in &booleanos {
        for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x && *y);
        }
    }

###Tabla de NOT

    println!("\nx\tnot x");
    println!("{:-<13}", ""); 
    for x in &booleanos {
        println!("{:?}\t{:?}", x,!x);
        
    }

###Tabla de XOR

    println!("\nx\ty\tx ^ y");
    println!("{:-<22}", ""); 
    for x in &booleanos {
        for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x ^ *y);
        }
    }

##Problemas al programar
En rust los arreglos deben definirse como en el siguiente ejemplo:
    
    let booleanos: [bool;2] =[false,true];
Dentro de la iteración, se deben utilizar asteriscos para poder realizar la operación lógica.
    
    for y in &booleanos {
            println!("{:?}\t{:?}\t{:?}", x,y,*x && *y);
        }
