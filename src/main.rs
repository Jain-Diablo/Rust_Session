fn main() {
    println!("Hello, world!");

    let mut a: u64 = 4294967296;
    println!("a: {}", a);
    a = 15;
    let b = 100;
    let c = 10000;
    println!("the number is {}  {}  {}", a, b, c);

    let char = 'G';
    println!("character is {}", char);

    let float_num = 3.14;

    println!("float number is {}", float_num);

    let my_bool = true;

    println!("My bool value is {} ", my_bool);

    const XY: i32 = 10;

    println!("const value  {}", XY);

    let my_name = String::from("GAUS");

    let lower_case = "Gaus".to_string();

    println!("My name {} and lowers case is {}", my_name, lower_case);

    let mut my_vec: Vec<u64> = Vec::new();
    my_vec.push(1);
    my_vec.push(2);

    let length_vector = my_vec.len();

    println!(
        "my vector/array is {:?} and its length is {}",
        my_vec, length_vector
    );

    struct Rectangle {
        length: u32,
        width: u32,
    }

    let my_rectangle = Rectangle {
        length: 10,
        width: 15,
    };

    println!(
        "length and width of my rectangle is {} and  {}",
        my_rectangle.length, my_rectangle.width
    );

    let c = sum(10, 20);

    println!("sum of two number is {}", c);

    let sub_return = sub(30, 10);

    println!("sub_return is {}", sub_return);

    let multiply_num = multiply(10, 10);

    println!("multiplication is {}", multiply_num);

    let my_string = "abcd".to_string();

    println!("print my_string {}", my_string);

    let print_me = true;

    if !print_me {
        println!("inside if")
    } else {
        println!("insied else")
    }
}

fn sum(a: u32, b: u32) -> u32 {
    let c = a + b;
    return c;
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn multiply(a: u32, b: u32) -> u32 {
    a * b
}