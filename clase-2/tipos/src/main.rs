fn main() {
    // punto flotantes
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // operaciones numéricas
    // adición
    let sum = 5 + 10;
    // sustracción
    let difference = 95.5 - 4.3;
    // multiplicación
    let product = 4 * 30;
    // división
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // resto
    let remainder = 43 % 5;

    // booleanos
    let t = true;
    let f: bool = false; // con tipo explícito

    // caracteres
    let c = 'z';
    let z: char = 'ℤ'; // con tipo explícito
    let heart_eyed_cat = '😻';

    // tuplas
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arreglos
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];

}