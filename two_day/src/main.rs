fn main() {
    // ===============元组=================
    let tup = (1, 0.2, 'x');
    println!("【 tup 】==> {:?}", tup);
    println!("【 tup 】==> {:?},{},{}", tup.0, tup.1, tup.2);
    // ===============数组=================
    let arr = [3, 5, 6];
    println!("【 arr 】==> {:?}", arr);
    let arr1 = [6; 3]; //[6,6,6]
    println!("【 arr1 】==> {:?}", arr1);
    // 读取数组中的元素
    println!("【 arr1 】==> {:?}", arr1[1]);
    // 超出数组长度报错
    // println!("【 arr1 】==> {:?}", arr1[4]);
    // ===============函数=================
    another_function(1, String::from("哈哈"));
    // 函数的参数必须声明类型
    fn another_function(param1: i32, param2: String) -> i32 {
        println!(
            "【 param1: i32, param2: String 】==> {:?},{:?}",
            param1, param2
        );
        // 函数返回值，不加分号相当于直接返回，return 233;
        233
    }
    // ===========循环========
    for num in 1..4 {
        println!("{}!", num);
    }

    let mut flag = 1;
    while flag < 4 {
        println!("{}!!", flag);
        flag += 1;
    }
}
