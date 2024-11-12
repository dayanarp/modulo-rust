fn main() {
    {                       // s no es valido aquí, aún no está declarado
        let s = "hola";     // s es valido desde aquí

        // Hacer algo con s
    }                       // este ámbito termina aquí, s ya no es valido

    // el tipo String
    let mut s = String::from("hola");
    s.push_str(", mundo!"); // push_str() agrega un literal a un String
    println!("{s}"); // Esto imprime "hola, mundo!"

    // moviendo datos
    let x = 5;
    let y = x;

    // moviendo datos con String
    let s1 = String::from("hola");
    let s2 = s1;

    // clone
    let s1 = String::from("hola");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    //ownership en funciones
    let s = String::from("hola");  // s aparece en el ámbito

    tomar_ownership(s);             // El valor de s se mueve a la función...
                                    // ... y ya no es valido aquí

    let x = 5;                      // x aparece en el ámbito

    hacer_una_copia(x);                  // x deberia moverse a la función,
                                    // pero i32 implementa Copy, entonces es
                                    // valido aún despues de llamar a la función

    // valores de retorno
    let s1 = da_un_ownership();         // da_un_ownership es llamado y
                                        // devuelve el valor de retorno
                                        // a s1

    let s2 = String::from("hola");     // s2 aparece en el ámbito

    let s3 = toma_y_devuelve(s2);  // s2 es movido a la función
                                        // toma_y_devuelve, que también
                                        // retorna el valor de s2 a s3 
}

// ownership en funciones
fn tomar_ownership(un_string: String) { // un_string aparece en el ámbito
    println!("{un_string}");
} // Aquí termina el ámbito, un_string es destruido con drop. 
  // La memoria es liberada.

fn hacer_una_copia(un_entero: i32) { // un_entero aparece en el ámbito
    println!("{un_entero}");
} // Aquí termina el ámbito, un_entero es destruido. Nada especial ocurre.


//valores de retorno
fn da_un_ownership() -> String {             // da_un_ownership mueve su
    // retorno a la función que la
    // llama

let un_string = String::from("tuyo");    // un_string aparece en el ámbito

un_string                                // un_string es retornado y
    // mueve su valor
}

// Esta función toma un String y devuelve uno
fn toma_y_devuelve(un_string: String) -> String { // un_string aparece 
         // en el ámbito

un_string  // un_string es retornado y mueve su valor
}