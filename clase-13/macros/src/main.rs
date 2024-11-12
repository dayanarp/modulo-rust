
//definiendo una macro declarativa
#[macro_export]
macro_rules! mi_debug {
    ($x:expr) => {
        println!("El valor de {} es: {:?}", stringify!($x), $x);
    };
}

fn main() {
    let valor = 10;
    mi_debug!(valor); // Expande a println!("El valor de valor es: {:?}", valor);

    calcular!(5 + 3); // Imprime "Suma: 8"
    calcular!(4 * 2); // Imprime "Multiplicación: 8"

    //  macros procedurales
    let persona = Persona {
        nombre: "Alice".to_string(),
        edad: 30,
    };
    println!("{:?}", persona); // Imprime los datos de `Persona` con formato `Debug`
}

// Macro declarativa con macro_rules
macro_rules! calcular {
    ($a:expr + $b:expr) => {
        println!("Suma: {}", $a + $b);
    };
    ($a:expr * $b:expr) => {
        println!("Multiplicación: {}", $a * $b);
    };
}

// Macros pocedurales 
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {

} 

#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u32,
}