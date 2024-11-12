use std::env;
use std::process;
use minigrep::Argumentos;

fn main() {
    let argumentos: Vec<String> = env::args().collect();

    let argumentos = Argumentos::new(&argumentos).unwrap_or_else(|err|{
        eprint!("Ha ocurrido un error: {}", err);
        process::exit(1);
    });

    println!("Buscando {}", argumentos.busqueda);
    println!("En el archivo {}", argumentos.nombre_archivo);

    if let Err(err) = minigrep::buscar(argumentos) {
        eprintln!("Ha ocurrido un error: {}", err);
        process::exit(1);
    }
}