use std::io;

fn main() {
    loop {
        mostrar_menu();
        let mut opcion = String::new();

        io::stdin().read_line(&mut opcion).expect("Error al leer la consola");

        let opcion : u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };
        match opcion {
            1 => println!("Has seleccionado la sumar"),
            2 => println!("Has seleccionado la restar"),
            3 => println!("Has seleccionado la multiplicar"),
            4 => println!("Has seleccionado la dividir"),
            0 => {
                println!("Saliendo del programa...");
                break; // Salir del bucle y del programa
            }
            _ => println!("Opción no válida"),
        }
    }
} 

fn mostrar_menu(){
    println!("Opciones de Calculadora");
    println!("1. Sumar");
    println!("2. Restar");
    println!("3. Multiplicar");
    println!("4. Dividir");
    println!("0. Terminar")
}

fn sumar(sum1: u32, sum2: u32){
    let suma = sum1 + sum2;
    println!("La suma de {} y {} es: {}",sum1, sum2, suma)
}

fn restar(sum1: u32, sum2: u32){
    let suma = sum1 - sum2;
    println!("La resta de {} y {} es: {}",sum1, sum2, suma)
}

fn multiplicar(sum1: u32, sum2: u32){
    let suma = sum1 * sum2;
    println!("El producto de {} y {} es: {}",sum1, sum2, suma)
}

fn dividir(sum1: u32, sum2: u32){
    let suma = sum1 / sum2;
    println!("La division de {} y {} es: {}",sum1, sum2, suma)
}