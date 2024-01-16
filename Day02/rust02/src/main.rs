fn main() {
    println!("Variables type");

    let a = 100;
    let b = 3.141592;

    println!("int   : {}", a);
    println!("float : {}", b);

    // 타입 지정
    let c: i16 = 12345;
    let d: f64 = 12.45726;
    let e: bool = false;

    println!("int   : {}", c);
    println!("float : {}", d);
    println!("bool  : {}", e);

    // 불변성
    let mut f = 200; // consider making this binding mutable: `mut f`
    f = f + 34;
    println!("sum   : {}", f); // cannot assign twice to immutable variable

    const PI: f64 = 3.14159265359;
    let dist = 5.0;
    let size = PI * dist * dist;
    println!("area  : {}", size);

    println!("====================!");

    let ch: char = 'X';
    println!("{}", ch);

    let cat: char = '😻'; // 와!
    println!("{cat}");

    // Shadowing
    let a = 10;
    println!("최초  : {a}");
    {
        let a = "hello";
        println!("내부  : {a}");

        let a = true;
        println!("내부에서 쉐도우: {a}");
    }
    println!("최종  : {a}");
}
