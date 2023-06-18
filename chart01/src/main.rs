

fn def_primitive_data_type() {
    let active:bool = true;
    let inactive:bool = false;

    let a:char = 'a';
    let b:char = 'b';
    let c:char = '∞';
    let great = '中';

    let prt:u8 = 182;  //u8
    let w:u8 = 70;     // u8
    let s:u8 = 23;     // u8
    let d:i8 = -128;   // i8

    let income_money:f32 = 100.20;
    let output_money:f32 = 20.80;
    let surplus_money:f64 = 79.40;
    // TODO ... fixed the issue
    // let _surplus_money:f32 = income_money-output_money;
    //
    // assert_eq!(_surplus_money.abs(), 79.40);

    let numbers: [i32; 8] = [4, 2, 4, 8, 3, 2, 4, 8];
    let grades: [i32; 4] = [20, 40, 34, 70];

    let mut xs = vec![1i32, 2];
    xs.push(3);

    let name = String::from("BYD");
    println!("{}", name);

    let mut s1 = String::from("你好");
    s1.push('！');
    s1.push_str(" 李先生");
    println!("{}", s1);

    let s = "A string";
    let ss = "A string".to_string();

    let s = String::from("Rust Web 编程");
    let rust = &s[0..5];
    let p = &s[5..];
    println!("{}{}", rust, p);

    let employee: (&str, i32, &str) = ("中国", 6, "美国");
}

fn main() {
    def_primitive_data_type();

}
