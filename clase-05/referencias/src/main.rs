fn main() {
    // referencias
    let s1 = String::from("hola");
    let len = calcular_longitud(&s1);
    println!("La longitud de '{s1}' es {len}.");

    // referencias mutables
    let mut s = String::from("hola");
    modificar(&mut s);
    {
        let r1 = &mut s;
    } // r1 se sale de su ámbito aquí, por lo que no hay problema 
      // si creamos otra referencia mutable
    let r2 = &mut s;
}

fn calcular_longitud(s: &String) -> usize { // es una referencia a un String
    s.len()
} // Aquí, s sale de ámbito. Pero como no tiene el ownership/la propiedad sino 
  // que s es solo un prestamo, no se destruye, se regresa al propietario, s1.

// referencias mutables
fn modificar(un_string: &mut String) {
    un_string.push_str(", mundo");
}