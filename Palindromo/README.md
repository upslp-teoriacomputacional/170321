## Datos personales

 - Universidad Politécnica de San Luis Potosí
 - Benjamín Loredo Amaya
 - 170321
 - Ingeniería en Tecnologías de la información
 - Teoría Computacional
 - Juan Carlos González Ibarra
 
## Objetivos
1.  Desarrollar un programa que verifique si una cadena es un palindromo
2. Aplicar un automata de pila.

## Como solucionaste el problema
Utilicé un automata de pila. Primero se llena la pila con las letras de la cadena; luego, para hacer una comparación, se extraen los valores de la pila. Misma que, al regirse por la secuencia LIFO, retornará las letras en el orden inverso al de la cadena. Los datos de la cadena y de la pila se comprueban uno a uno para saber si son iguales y , por lo tanto, la cadena es un palindromo.

## Programacion de la pila
     
     struct Stack<T> {
     stack: Vec<T>,
    }

     impl<T> Stack<T> {
      fn new() -> Self {
        Stack { stack: Vec::new() }
      }



      fn pop(&mut self) -> Option<T> {
        self.stack.pop()
      }

      fn push(&mut self, item: T) {
        self.stack.push(item)
      }

      fn is_empty(&self) -> bool {
        self.stack.is_empty()
      }

    }
   
## Funciones
### Funcion de comparacion
Devuleve 0 si son iguales, 1 si no lo son.

    fn comparacion(character1: char,character2: char) -> i32 {
      //Compara los caracteres
      if character1==character2{
        return 0;
      }else{
        return 1;
      }
    }

### Funciones para mostrar la tabla en consola

    fn encabezado(){
        println!("|  Edo. Actual |Caracter1 |  Caracter2  |Edo. Siguiente |");
        body();
    }

    //definimos la funcion contenido donde guarda cada valor despues de encontrarlo en un ciclo
    fn contenido(estadosig: i32,character1: char,character2: char,estado: i32){ 
          println!("|    {}    |    {}    |   {}   |     {}      |",estadosig,character1,character2,estado);
      body();
    }

    //solo muestra la linea que se repetira cada vez que la mandemos a llamar
    fn body(){
        println!("+--------------+---------+-----------+---------------+");
    }
### Funcion main

    fn main() {

      let mut stack: Stack<char> = Stack::new();
      let tabla= [[0,1,0],[1,1,1]];
      let mut estado: i32 = 0;

      println!("+-------------------------------------+
        |    Ingrese una cadena a evaluar:    |
        +-------------------------------------+");

      //Lee la cadena
      let mut cadena = String::new();		
      io::stdin().read_line(&mut cadena);
      body();
      encabezado();

      //Llena la pila con el contenido de la cadena
      for  character in cadena.chars(){
        //omite los espacios en la cadena
        if character!=' '&&character!='\n'{
          stack.push(character);
        }
      }

      //Verifica que la cadena no este vacía
      if stack.is_empty(){

        println!("La cadena esta vacia");

      }else{

        //Un ciclo que se ejecutara hasta vaciar la pila
        while !stack.is_empty(){
          let estadosig: i32 =estado;
          let character1=cadena.remove(0);

          //omite los espacios en la cadena
          if character1!=' '{

            let character2 = stack.pop(); //saca un elemento de la pila
            let compara=comparacion(character1,character2.unwrap());
            estado=tabla[estado as usize][compara as usize];

            //Si el estado es 1 significa que no coinciden los caracteres
            if estado==1{
              println!("|    {}    |  {}    | {} |     {}      |",estadosig,character1,character2.unwrap(),estado);
              body();
              println!("|              No es un palindromo :(                   |
      +----------------------------------------------------+");
              process::exit(1);
            }

            contenido(estadosig,character1,character2.unwrap(),estado);
          }   
        }

        //Si esta en el estado de aceptación al terminar la ejecución
        if estado==0{
            println!("|     {}      |         |Fin Cadena |               |",estado);
            body();
            println!("|           La cadena es un palindromo                       |
        +----------------------------------------------------+");
        }

        //Si no esta en el estado de aceptación
        if estado!=0{
                println!("|              Cadena No Valida :(                   |
        +----------------------------------------------------+");
        }
      }

    }

##Problemas al programar
Si no utilizaba uno de los métodos de la pila, me salía error. Al parecer, debo utilizar todos los métodos declarados. El problema es que aun no programaba su uso.

