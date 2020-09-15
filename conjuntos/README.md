## Datos personales

 - Universidad Politécnica de San Luis Potosí
 - Benjamín Loredo Amaya
 - 170321
 - Ingeniería en Tecnologías de la información
 - Teoría Computacional
 - Juan Carlos González Ibarra
 
 ## Objetivos
En el programa se utilizan varias funciones sobre los conjuntos en el lenguaje Rust.
## Como solucionaste el problema
Use la libreria de HashSet. Con esa libreria puedo usar métodos que facilitan mi trabajo con conjuntos.
## Explicación del código

    let mut conjuntoA = HashSet::new();
    let mut conjuntoB = HashSet::new();
    let mut conjuntoC = HashSet::new();
    
    //Llena los conjuntos
    for x in 1..6{
        conjuntoA.insert(x);
    }
    for x in 3..8{
        conjuntoB.insert(x);
    }
    //Muestra los conjuntos
    println!("Conjunto de A: {:?}", conjuntoA);
    println!("Conjunto de B: {:?}", conjuntoB);
    println!("Conjunto de C: {:?}", conjuntoC);

En la primera parte del código declaro los conjuntos que voy a utilizar. Lo hago utilizando HashSet::new para generar el conjunto.
Luego lleno los conjuntos utilizando un ciclo for.
Al final, muestro los conjuntos en la consola.

    println!("\n Eliminar un elemento");
    conjuntoA.remove(&3);
    println!("Conjunto A: {:?}", conjuntoA);

Ahí elimino un elemento del conjunto utilizando el método remove propio de la variable.

    println!("\n Limpiar el conjunto completo");
    conjuntoA.clear();
    println!("Conjunto A: {:?}", conjuntoA);
    for x in 1..6{
        conjuntoA.insert(x);
    
Luego elimino todo el contenido del conjunto utilizando el método clear. Lo lleno de nuevo después de limpiarlo.

    //Union de conjuntos
    println!("\nUnion de conjuntos");
    println!("Union del conjunto A y  conjunto B: {:?}", conjuntoA.union(&conjuntoB));

    //Interseccion de conjuntos
    println!("\n Interseccion de conjuntos");
    println!("Interseccion de A y B: {:?}", conjuntoA.intersection(&conjuntoB));

    //Diferencia de conjuntos
    println!("\nDiferencia de conjuntos");
    println!("Diferencia de A y B: {:?}", conjuntoA.difference(&conjuntoB));

    //Diferencia simetrica de conjuntos
    println!("\nDiferencia simetrica de conjuntos");
    println!("Diferencia A y B: {:?}", conjuntoA.symmetric_difference(&conjuntoB));
    println!("Diferencia B y A: {:?}", conjuntoB.symmetric_difference(&conjuntoA));
    println!("Diferencia A y C: {:?}", conjuntoA.symmetric_difference(&conjuntoC));
    println!("Diferencia B y C: {:?}", conjuntoB.symmetric_difference(&conjuntoC));

En este segmento se realiza la unión, diferencia, intersección, y diferencia simétrica de los conjuntos. Todos usan un método propio de la variable. La función devuelve el resultado de la operación en forma de conjunto.

    //subconjuntos 
    println!("\nSubconjuntos ");
    println!("\nA es subconjunto de B?: {:?}", conjuntoA.is_subset(&conjuntoB));
    println!("\nB es subconjunto de A?: {:?}", conjuntoB.is_subset(&conjuntoA));

    //Superconjuntos
    println!("\nSuperconjuntos");
    println!("\nA es supersubconjunto de B?: {:?}", conjuntoA.is_superset(&conjuntoB));
    println!("\nB es supersubconjunto de A?: {:?}", conjuntoB.is_superset(&conjuntoA));
Es la última parte del código. Se utiliza un método que verifica si el conjunto es subconjunto del otro. Al igual que en los superconjuntos.
Este método devuelve un valor que puede ser cierto o falso.


## Problemas al programar
En algunas ocasiones cometí el error de no colocar el signo "?" en esta sentencia.

        println!("Conjunto A: {:?}", conjuntoA);
También encontré que el compilador me sugiere eliminar el mut en la definición de variables. Eso resulta en un error

        let mut conjuntoA = HashSet::new();

