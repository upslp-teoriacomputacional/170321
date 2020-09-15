/* ----------------------------------------------------------------------
 *  Universidad Politecnica de San Luis Potosí
 *  Ingeniería en Tecnologías de Información
 *
 *  Materia: Teoría Computacional
 *  Profesor: Juan Carlos González Ibarra
 *  Nombre:    Benjamín Loredo Amaya
 *  Matricula: 170321
 *  
 *  Escrito:       14/09/2020
 *  Ultima actualización:  15/09/2020
 *----------------------------------------------------------------------*/
use std::collections::HashSet;

fn main(){

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

    //Eliminar un dato del conjunto

    println!("\n Eliminar un elemento");
    conjuntoA.remove(&3);
    println!("Conjunto A: {:?}", conjuntoA);

    //limpiar todo el conjunto

    println!("\n Limpiar el conjunto completo");
    conjuntoA.clear();
    println!("Conjunto A: {:?}", conjuntoA);
    for x in 1..6{
        conjuntoA.insert(x);
    }   

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

    //subconjuntos 
    println!("\nSubconjuntos ");
    println!("\nA es subconjunto de B?: {:?}", conjuntoA.is_subset(&conjuntoB));
    println!("\nB es subconjunto de A?: {:?}", conjuntoB.is_subset(&conjuntoA));

    //Superconjuntos
    println!("\nSuperconjuntos");
    println!("\nA es supersubconjunto de B?: {:?}", conjuntoA.is_superset(&conjuntoB));
    println!("\nB es supersubconjunto de A?: {:?}", conjuntoB.is_superset(&conjuntoA));


}