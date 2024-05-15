fn main() {
    println!("Hello, world!");

    // 배열
    let arr: [i32; 3] = [1,2,3];
    println!("{}", arr[0]); // 배열 인덱싱
    println!("{:?}", arr); // 전체 배열 출력

    // 튜플
    let tup: (i32, bool, char) = (13, false, 'S');
    let spiderman: (&str, &str) = ("Tom", "Holand");

    println!("{}", tup.2);
    println!("{:?}", tup);
    // println!("{}", &spidernam.1);
    
}
