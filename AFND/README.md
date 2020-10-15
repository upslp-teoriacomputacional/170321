## Datos personales

 - Universidad Politécnica de San Luis Potosí
 - Benjamín Loredo Amaya
 - 170321
 - Ingeniería en Tecnologías de la información
 - Teoría Computacional
 - Juan Carlos González Ibarra
 
## Objetivos
1.  Desarrollar un AFND que valide la entrada de datos en el siguiente formato: a*ba*

## Como solucionaste el problema
Utilicé un array bidimensional para crear la tabla de estados. 
Para las comparaciones en la función caracter tuve que utilizar operadores lógicos.
Realicé cambios en la tabla anterior para que pueda validar este formato.
### Librerias 
    use std::process;
    use std::io;
    use std::str;
### Función para comprobar caracteres
 
    //Definimos la funcion caracter 
    fn caracter(character: char) -> i32 {
        let mut Fin: char = '\n';
        let mut a: char = 'a';
        let mut b: char = 'b';
	
	    //Compara con los caracteres esperados
        if character==a{
				return 0;            
        }
        else{
            if character==b {
                return 2;
                }
            else{
		if character == Fin {
                    return 1;
                }
            }

            //si no es ninguno de los valores esperados, no es válido.
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
    
    //Este es la tabla de transiciones del automata AFND creado
    let tabla= [[0,4,1],[2,3,4],[2,3,4]];
      
    let mut estado: i32 = 0;
    let mut aux: i32 = 0;
    
    println!("+-------------------------------------+
    |    Ingrese una cadena a evaluar:    |
    +-------------------------------------+");
    
    //Lectura de datos
    let mut cadena = String::new();		
    io::stdin().read_line(&mut cadena);
    body();
    encabezado();
    let mut estadosig: i32;

    //ciclo para recorrer la cadena
    for  character in cadena.chars(){
        //Para contar los caracteres recorridos
        aux=aux+1;

        //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        let mut charcaracter= caracter(character);

        //guardamos en estado el valor obtenido en la tabla segun las cordenadas que recibio anteriormente
        estado=tabla[estado as usize][charcaracter as usize];
        estadosig=estado+1;
        if charcaracter == 0{
      	    simbolo = "a*".to_string();
            if estado==1 {estadosig=1}; //Corrige un error en la impresión de la tabla
      	}
      	else if charcaracter == 1{
      	    simbolo = "b".to_string();
      	}
      	else if charcaracter == 2{
      	    simbolo = "Fin".to_string();

      	}
      	
    
        //Si el valor obtenido es un estado no valido, imprimimos cadena no valida
        if estado==4{
            println!("|    {}    |  {}    | Error |     {}      |",estadosig,character,estado);
            body();
            println!("|              Cadena No Valida :(                   |
    +----------------------------------------------------+");
            process::exit(1);
        }
        //si el estado es 3 y se ha llegado al final de la cadena, es una cadena de aceptacion
        if (estado==3) && aux as usize==cadena.chars().count(){
            println!("|     {}      |         |Fin Cadena |               |",estado);
            body();
            println!("|                Cadena Valida                       |
        +----------------------------------------------------+");
            process::exit(1);

        }
        //si no es fin de cadena y el estado es 3, lo hace 2 para continuar correctamente 
        if estado==3 {estado=2}
        
        contenido(estado,character,&simbolo,estadosig);
    }
 
    }
 ## Problemas y soluciones al programar
 No encontré como utilizar variables globales, así que tuve que definir la variable dependiendo de lo que devolviera la función caracter.
    
    //llamamos al metodo para saber si es a* o b
        if charcaracter == 0{
      	    simbolo = "a*".to_string();
            if estado==1 {estadosig=1}; //Corrige un error en la impresión de la tabla
      	}
      	else if charcaracter == 1{
      	    simbolo = "b".to_string();
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
El arreglo debe contener valores del mismo tipo, asi que utilicé solamente numeros

    //Este es la tabla de transiciones del automata AFND creado
    let tabla= [[0,4,1],[2,3,4],[2,3,4]];
En rust se obtiene la extensión de la cadena con una función que devuelve un vaor usize, por eso realicé una conversión en este if
	
	//si el estado es 3 y se ha llegado al final de la cadena, es una cadena de aceptacion
        if (estado==3) && aux as usize==cadena.chars().count(){
