fn main() {

    println!("{}", "Hello World");

    println!("{}", "Eko");

    println!("{}", "Budi");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Steven";

    println!("Hello {}", name)
}

#[test]
fn test_mutable() {
    let mut name = "Steven";
    println!("Hello {}", name);

    name = "George";
    print!("Hello {}", name);
}

#[test]
fn static_typing() {
    let mut name = "Steven";
    println!("Hello {}", name);

    name = "George";
    print!("Hello {}", name);
}

/*
ini code terkait shadowing
*/

#[test]
fn shadowing() {
    let name = "Steven";
    println!("Hello {}", name);

    let name = 10;
    print!("Hello {}", name);
}


#[test]
fn explicti() {
    let age = 10;
    println!("Hello {}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("Hello {}", a);

    let b: f32 = 10.5;
    print!("hello {}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    print!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 10000000000;
    let e: i8 = d as i8;

    println!("{}", e);
}

#[test]
fn number_assignment() {
    let mut a = 10;
    println!("{}", a);
    a += 10;
    println!("{}", a);
    a -= 10;
    println!("{}", a);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result = a < b;
    println!("{}", result);
}

#[test]
fn boolead_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhor = nilai_akhir >= 75;

    let result = lulus || lulus_nilai_akhor;
    println!("{}", result);
}

#[test]
fn tuple() {
    let mut data = (10, 25.10, true);
    println!("{:?}", data );

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.4;
    data.2 = false;

    println!("{:?}", data);
}

fn unit() {
    println!("helloo")
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let mut array = [1,2,3,4,5];
    println!("{:?}", array );

    let a = array[0];
    let b = array[4];
    println!("{} {}", a, b );

    array[4] = 8;
    array[3] = 58;
    array[2] = 500;

    println!("{:?}", array);
    
    let length = array.len();
    println!("{:?}", length );
}

#[test]
fn two_dimensional_array() {
    let matrix = [
        [3,2,6],
        [5,6,6],
        [6,7,4]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 300;

#[test]
fn constant() {
    const MINIMUM: i32 = 20000;

    println!("{} {}", MINIMUM, MAXIMUM );
}

#[test]
fn variable_scope() {
    let hakim = 1;

    {
        println!("inner cycle {}", hakim);
        let baja_ringan = 60;
        println!("inner baja ringan {}", baja_ringan);
    }

    println!("outer cycle {}", hakim);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Kurniawan");

    println!("{} {}", a, b)
}

fn function_b() {
    let a = 10;
    let b = String::from("Eko");

    println!("{} {}", a, b)
}
