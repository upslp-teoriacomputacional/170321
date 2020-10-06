## Datos personales

 - Universidad Politécnica de San Luis Potosí
 - Benjamín Loredo Amaya
 - 170321
 - Ingeniería en Tecnologías de la información
 - Teoría Computacional
 - Juan Carlos González Ibarra
 
## Objetivos
1.  Desarrollar un automata finito que valide la entrada de datos en el siguiente formato: n operacion n

## Como solucionaste el problema
Utilicé un array bidimensional para crear la tabla de estados. 
Para las comparaciones en la función caracter, tuve que utilizar operadores lógicos.
### Librerias 
    use std::process;
    use std::io;
    use std::str;
### Función para comprobar caracteres
 
    //Definimos la funcion caracter 
        fn caracter(character: char) -> i32 {
            let mut Fin: char = '\n';
            //comparamos si es digito u operador

           if character.is_digit(10){
           return 0;            
           }
           else{
          //Comproueba si es un operador valido
               if character=='-'||character=='/'||character=='+'||character=='*' {
                   return 1;
                   }
               else{
           if character == '\n' {
                       return 2;
                   }
               }

               //si no es ni un digito ni un operador entonces es un caracter no validp
               println!("Error el caracter: {} no es valido", character);
               process::exit(1);
           }
       }
### Funciones para imprimir la tabla en la consola
     //definimos al la funcion  encabezado
    fn encabezado(){
        println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
        body();
    }
    
    //definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
    fn contenido(estadosig: i32,character: char,simbolo: &str,estado: i32){ 
            if character=='\0'{ //Comprueba si no es el salto de linea
            println!("|    {}    |        |   {}   |     {}      |",estadosig,simbolo,estado);
			}else{
				 println!("|    {}    |    {}    |   {}   |     {}      |",estadosig,character,simbolo,estado);
			}
			body();
    }
    
    //solo muestra la linea que se repetira cada vez que la mandemos a llamar
    fn body(){
        println!("+--------------+---------+-----------+---------------+");
    }
### Función main
       //MAIN
    fn main(){
        let mut  simbolo: String = "".to_string();
        let mut  Fin: String = "".to_string();

        //Este es la tabla de transiciones del automata AFD creado
        let tabla= [[1,69,69],[69,2,69],[3,69,69],[69,69,65]];

        let mut estado: i32 = 0;

        println!("+-------------------------------------+
        |    Ingrese una cadena a evaluar:    |
        +-------------------------------------+");

        //Lectura de datos
        let mut cadena = String::new();		
        io::stdin().read_line(&mut cadena);
        body();
        encabezado();

        //ciclo para recorrer la cadena
        for  character in cadena.chars(){
            let mut estadosig: i32 =estado;

            //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
            let mut charcaracter= caracter(character);

            if charcaracter == 0{
               simbolo = "Digito".to_string();
           }
           else if charcaracter == 1{
               simbolo = "Operador".to_string();
           }
           else if charcaracter == 2{
               simbolo = "Fin".to_string();
           }

            //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
            estado=tabla[estado as usize][charcaracter as usize];

            //Si el valor obtenido es una E imprimimos cadena no valida
            if estado==69{
                println!("|    {}    |  {}    | Error |     {}      |",estadosig,character,estado);
                body();
                println!("|              Cadena No Valida :(                   |
        +----------------------------------------------------+");
                process::exit(1);
            }
            contenido(estadosig,character,&simbolo,estado);
        }

        //si el estado es 65 es una cadena de aceptacion
        if estado==65{
            println!("|     {}      |         |Fin Cadena |               |",estado);
            body();
            println!("|                Cadena Valida                       |
        +----------------------------------------------------+");
        }
       //al concluir si el estado no es 65 que es el de aceptacion imprimimos cadena no valida    
        if estado!=65{
                println!("|              Cadena No Valida :(                   |
        +----------------------------------------------------+");
        }
    }
 ## Problemas y soluciones al programar
 No encontré como utilizar variables globales, así que tuve que definir la variable dependiendo de lo que devolviera la función caracter.
    
    //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter= caracter(character);
        
        if charcaracter == 0{
      	    simbolo = "Digito".to_string();
      	}
      	else if charcaracter == 1{
      	    simbolo = "Operador".to_string();
      	}
      	else if charcaracter == 2{
      	    simbolo = "Fin".to_string();
      	}
En una de los if en la función caracter (donde se comprueba el final de la cadena) cambie el "" por "\n", porque de otro modo no lo aceptaba.
    
    if character=='\0'
Cuando se buscan los valores en la tabla, se debe realizar una conversión para que funcione
     
     estado=tabla[estado as usize][charcaracter as usize];
Para imprimir la tabla correctamente, tuve que emplear un if para que verifique que character no sea el salto de linea
    
    fn contenido(estadosig: i32,character: char,simbolo: &str,estado: i32){
            if character=='\n'{  //Comprueba si no es el salto de linea
            println!("|    {}    |        |   {}   |     {}      |",estadosig,simbolo,estado);
			}else{
				 println!("|    {}    |    {}    |   {}   |     {}      |",estadosig,character,simbolo,estado);
			}
			body();
    }
El arreglo debe contener valores del mismo tipo, asi que cambie las letras por sus valores en ascii

    let tabla= [[1,69,69],[69,2,69],[3,69,69],[69,69,65]];
