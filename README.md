# rustcode

## 语法和基本概念

### 变量

1. 变量使用let关键字声明
2. 变量可变性
3. 命名方式：蛇形命名

```rust
//变量默认不可变，
let a = 1;
//想改变就要加mut关键字
let mut a = 1；
a = 2；
```

### 常量

1. 常量使用const关键字声明
2. 常量声明必须显式的指明类型
3. 常量可以在任何作用域声明，常量在哪都能用
4. 约定俗成，常量名称全大写const MAX_POINTS: u32 = 100_000;

### 隐藏

1. 同名变量可以隐藏前一个变量，
2. 隐藏时可以变更变量类型

```rust
let a = 1;
let a = 2;
let a = "hello"; 
```

### 数据类型

#### 标量类型 scalar

1. 整数类型

    |长度|有符号|无符号|
    |---|---|---|
    |8-bit|i8|u8|
    |16-bit|i16|u16|
    |32-bit|i32|u32|
    |64-bit|i64|u64|
    |arch|isize|usize|

2. 整数字面值

    | 整数字面量  | 示例  |
    |---|---|
    |  Decimal |  100_0000_000 |
    | Hex  | 0xff  |
    | Octal  | 0o77  |
    | Binary  | 0b11111  |
    | Byte（u8 only）  | b'A'  |

3. 整数的默认推到类型为i32

4. debug模式下，整数溢出会panic

5. 浮点类型：f32、f64

6. 浮点数默认推到类型为f64

7. 布尔类型bool

8. 字符类型char： 占4字节，表示unicode字符。

#### 复合类型 compound

1. 元组tuple
   1. 定义元组：let tup: (i32, f64, u8) = (500, 6.4, 1);
   2. 类型可以不同，长度固定
   3. 使用点引用元组内容，tup.0,tup.1
2. 数组array
   1. 定义数组：let a = [1, 2, 3, 4, 5];
   2. 访问数组元素：a[0]; 该方式可能数组越界导致panic
   3. 类型相同，长度固定

### 函数

1. 命名方式：蛇形命名
2. 函数参数
3. 语句：执行操作，但不返回值
4. 表达式：会进行计算并返回一个值作为结果的指令
5. 返回值： -> 函数最后一个表达式，作为函数返回值

### 注释

```rust
// 这是个单行注释
/*
    这里是块注释
*/
```

### 控制流

```rust
//
fn main(){
    let n:i32 = 3;
    // if 语句的基本用法
    if n > 5 {
        println!("number greater than 5");
    } else if n < 5 {
        println!("number lower than 5");
    } else {
        println!("number equal 5");
    }


    //判断中，不允许将数组转换为bool类型
    //if n { println!("hello");}

    // if语句是一个表达式，返回值可以给变量赋值
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    //loop语句基本用法,循环，使用break返回
    let i = 1;
    loop {
        if i > 10 {
            break;
        }
        println!("helloworld: {i}");
        i = i + 1;
    }

    // loop表达式，通过break返回值
    let i2:i32 = loop {
        break 100;
    }

    // while循环
    let mut n1 = 3；
    while n1 != 0 {
        println!("number: {n1}");
        n1 = n1 - 1;
    }
}
```














变量、数据类型和常量
控制流程（条件语句、循环等）
函数和闭包
所有权和借用规则
1. 命令行参数解析库
2. 日志库