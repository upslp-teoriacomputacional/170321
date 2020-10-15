/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    Benjamín Loredo Amaya
 *  Matricula: 170321
 *  
 *  Escrito:       15/10/2020
 *  Ultima actualización:  15/10/2020
 *----------------------------------------------------------------------*/

use std::process;
use std::io;
use std::str;


    //Definimos la funcion caracter 
    fn caracter(character: char) -> i32 {
        let mut Fin: char = '\n';
        let mut a: char = 'a';
        let mut b: char = 'b';
        //comparamos si es digito u operador

        if character==a{
				return 0;            
        }
        else{
		//Comproueba si es un operador valido
            if character==b {
                return 2;
                }
            else{
				if character == Fin {
                    return 1;
                }
            }

            //si no es ni un digito ni un operador entonces es un caracter no validp
            println!("Error el caracter: {} no es valido", character);
            process::exit(1);
        }
    }
    
    //definimos al la funcion  encabezado
    fn encabezado(){
        println!("|  Edo. Actual |Caracter |  Simbolo  |Edo. Siguiente |");
        body();
    }
    
    //definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
    fn contenido(estadosig: i32,character: char,simbolo: &str,estado: i32){
            if character=='\n'{  //Comprueba si no es el salto de linea
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
            if estado==1 {estadosig=1};
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
        //si no es fin de cadena y el estado es 3, lo hace 2 para continuar
        if estado==3 {estado=2}
        
        contenido(estado,character,&simbolo,estadosig);
    }
 
    
   
}
