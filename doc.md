## Why Rust?

- High-level language features without performance penalties
- Program behaviors can be enforced at compile time
- Enhanced program reliability
- Built-in dependency management, similar to npm
- Quickly growing ecosystem of libraries
- Friendly & welcoming developer community

## Technical Rust Goodies

- First-class multithreading
    - Compiler error to improperly access shared data
- Type system:
    - Can uncover bugs at compile time
    - Makes refactoring simple
    - Reduces the number of tests needed
- Module system makes code separation simple
- Adding a dependency is 1 line in a config file
- Tooling:
    - Generate docs, lint code, auto format

## Data Types

- Memory only stores [binary data](https://)
    - Anything can be represented in binary
    - Program determines what the binary represents
- Basic types that are universally useful are provided by the language

## Basic Data Types

- Boolean
    - true, false
- Integer
    - 1, 2, 50, 99, -2
- Double / Float
    - 1.1, 5.5, 200.0001, 2.0
- Character
    - 'A', 'B', 'c', '6', '$'
- String
    - "Hello", "world", "this is a string", "it's 101"

> ♟️ Anything can be represented with binary <br>
  ♟️ Basic data types are:
    `boolean`, `integer`, `double & float`, `character`, `string`

## What is a variable?

- Assign data to a temporary memory location
    - Allows programmer to easily work with memory
- Can be set to any value & type
- Immutable by default, but can be mutable
    - `Immutable`: cannot be changed
    - `Mutable`: can be changed

> Memory is essentially data; it serves as a space for storing information.

## Examples

```rust
let number = 2;
let hello = "hello";
let x = 'x';
let my_half = 0.5;
let mut my_name = "Mohammed";
let learn_program = true;
let your_half = my_half;
```

> 🚀 Variables make it easier to work with data <br>
  🚀 Variables can be assigned to any value This include other variables <br>
  🚀 Immutable by default

## What are functions?

- A way to encapsulate program functionality
    - Optionally accept data
    - Optionally return data
- Utilized for code organization
    - Also makes code easier to read

## Anatomy of a function

### Structure
```rust
fn fun_name(parameters: type) -> return_type {
    return something
}
```

### Example
```rust
fn add(a: 132, b: i32) -> i32 {
    a + b
}
```

## Using a function

```rust
fn add(a: 132, b: i32) -> 132 {
    a + b
}

let x = add(1, 1);
let y = add(3, 0);
let z = add(x, 1);
```

> ✴️ Functions encapsulate functionality <br>
✴️ Useful to organize code <br>
✴️ Can be executed by "calling" the function <br>
&nbsp;&nbsp; &nbsp; &nbsp; &nbsp;〽️ Parameters determine what data a function can work with <br>
✴️ Optionally "returns" data <br>
&nbsp;&nbsp; &nbsp; &nbsp; &nbsp;〽️ Data sent back from the function

## The println macro

- Macros expand into additional code
- println "Prints" (displays) information to the terminal
- Useful for debugging

```rust
let x = 101;

println!("hello world");
println!("{:?}", x);
println!("{:?} {:?}", x, x);
println!("the meaning is {:?}", x);

// also use
println!("{x:?}");
// and
println!("{x}");
```

## Macros use an exclamation point to call/invoke

- Generate additional Rust code
- Data can be printed using println!:
    - `{:?}`
    - `{varname:?}`

## Execution Flow

- Code executed line-by-line
- Actions are performed & control flow may change
- Specific conditions can change control flow
    - ► `if`
    - ► `else`
    - ► `else if`

## Example - Nested if..else

```rust
let a 99;
if a > 99 {
    if a > 200 {
        println! ("Huge number");
    } else {
        println! ("Big number");
    }
} else {
    println! ("Small number");
}
```

### Example Simple Flow

```rust
+--------+
| Start  |
+---+----+
    |                                                           
    v                                   
+---+------------+
| Declare x = 1  |
+---+------------+
    |
    v                                 let x = 32;
+---+------------+                    let y = 32;
| Declare y = 2  |                    let z = 32;
+---+------------+
    |
    v
+---+------------+
| Declare x = 3  |
+---+------------+
    |
    v
+---+----+
| End    |
+--------+
```

### Example if..else

```rust
+-----------------------+
|        Start          |
+-----------------------+
         |                                                           
         v   
     +----------+
     |  x = 99  | (Assignment)
     +----------+
         |                            let x = 99;
         v
     +----------+                     if x > 99 {
     |  x > 99? | (Decision)              println! ("Big number");
     +----------+                     } else {
         |                                println! ("Small number");
         v                            }
     /        \
    Yes        No
     v        v
+----------+ +----------+
| Big      | | Small    |
| number   | | number   |
+----------+ +----------+
         |
         v
+-----------------------+
|        End            |
+-----------------------+
```

### Example Nested if..else

```rust
+-----------------------+
|        Start          |
+-----------------------+
           |
           v
      +----------+
      |  x = 99  | (Assignment)
      +----------+
           |
           v
      +----------+
      |  x > 99? | (Decision)
      +----------+
        /        \                    let x = 99;
      Yes         No
       |           |                  if x > 99 {
       v           v                      if x > 200 {
+----------+     +----------+                 println! ("Huge number");
| x > 200? |     | Small    |             } else {
|(Decision)|     | number   |                 println! ("Big number");
+----------+     +----------+             }
  /         \               |         } else {
Yes          No             |             println! ("Small number");
 |            |             |         }
 v            v             |
+----------+ +----------+   |
| Huge     | | Big      |   |
| number   | | number   |   |
+----------+ +----------+   |
      |            |        |
      v            v        |
+-----------------------+   |
|        End            |<--/
+-----------------------+   
```

### Example if..else if..else

```rust
+-----------------------+
|        Start          |
+-----------------------+
         |
         v    
    +----------+
    |  x = 99  | (Assignment)
    +----------+
         |
         v                                                   
    +----------+
    |  x > 200?| (Decision)
    +----------+
     /        \                       let x =  99;
    Yes       No
     |        |                       if x > 200 {
     v        v                           println! ("Huge number");
+----------+ +----------+             }else if x > 99 {
| Huge     | | x > 99?  |                 println! ("Big number");
| number   | |(Decision)|             } else {
+----------+ +----------+                 println! ("Small number");
|             /     \                 }
|            Yes     No     
|           /         \ 
|          /           \  
|   +----------+ +----------+
|   | Big      | | Small    |
|   | number   | | number   |
|   +----------+ +----------+
|        |            |    
|        v            v       
|   +-----------------------+
\-->|        End            |
    +-----------------------+
```

> ✴️ Code executes line-by-line <br>
✴️ This can be changed using `if` <br>
✴️ Try to always include `else`, unless there truly is no alternative case

## Repetition

- Called `looping` or `iteration`
- Multiple types of loops
 - `loop` infinite loop
 - `while` conditional loop

### Loop & While loop

```rust
+-----------------------+
|        Start          |                     // loop
+-----------------------+                     let mut z = 0;
          |
          v                                   loop {
     +----------+                                 if z == 5 {
     |   z = 0  | (Assignment)                        break;
     +----------+                                 }
          |                                       println!("{:?}", z);
          v                                       z = z + 1;
+-----------------------+<-------------\      }
|         Loop          | (Loop Start) |
+-----------------------+              |
          |                            |
          v                            |
     +----------+                      |
     |  Print z | (Output)             |      // while loop
     +----------+                      |      let mut x = 0;
          |                            |
          v                            |      while x != 5 {
     +----------+                      |          println!("{:?}", x);
     |  z == 5? | (Decision)           |          x = x + 1;
     +----------+                      |      }
        /     \                        |
       Yes     NO                      |
      /         \                      |
     /           \                     |
+-----------+ +-----------+            |
|  break    | | z = z + 1 |  continue  |
|           | |(Increment)|----------->/
+-----------+ +-----------+
        |
        v  
     +------------------+
     |       End        |
     +------------------+
```

> ♟️ Repetition can be performed using loops <br>
&nbsp;&nbsp; &nbsp; &nbsp; &nbsp;♟️ While loop <br>
&nbsp;&nbsp; &nbsp; &nbsp; &nbsp;♟️ Infinite loop <br>
♟️ Both types of loops can exit using "break"

## Rust installation

### Installing Rust on Linux

1. Install Rustup: Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Verify Installation: In the terminal, type `rustc --version` to verify Rust installation. Check Cargo with `cargo --version`.

### Installing Rust on macOS

1. Install with Homebrew: In a terminal, run:

```bash
brew install rust
```

2. Verify Installation: In the terminal, type `rustc --version` to verify Rust installation. Check Cargo with `cargo --version`.

## Creating a Rust Project

```bash
cargo init hello_world
cd hello_world
```

**Exploring Project Files:** Inside the project directory, you'll find the following files:
- `Cargo.toml`: This file contains metadata about the project and its dependencies. You can specify dependencies, project name, version, and other configurations here.
- `src` directory: This directory contains your project's source code files. The `main.rs` file is the entry point for your Rust application.

## Running a Rust Project

1. **Build the Project:** To compile your Rust project, run the following command in the project directory:
```bash
cargo build
```

This command compiles your project and generates the executable binary file.

2. **Run the Project:** Once the project is built successfully, you can run the executable binary by executing:

```bash
cargo run
```

This command builds and executes the project. You'll see the output of your Rust program in the terminal.

## Basic Hello, World! Program

Now let's add the traditional "Hello, World!" program to your Rust project. Open the `main.rs` file located in the `src` directory and replace its contents with the following code.

```rust
fn main() {
    println!("Hello, world!");
}
```

This simple program prints "Hello, world!" to the console.

- **Run the Program:** After adding the "Hello, World!" program, you can run it using the `cargo run` command as described above. You should see the "Hello, world!" message printed in the terminal.

```bash
cargo run
```

### Output
```bash
Hello, world!
```

> Congratulations! 🎉 You have successfully created a Rust project, built it, and executed it. You can now continue to develop your project by adding more code to the `main.rs` file or by creating additional Rust source files in the `src` directory.

## Code Comments

- Comments are used to explain code in plain language.
- They help other developers understand your code.
- Comments are ignored by the compiler.
- In Rust, single-line comments start with `//`.
- Multi-line comments start with `/* and end with */`.
- Prefer clear variable names that express their purpose over using comments.
- Keep comments concise and use them only when needed to explain complex parts of the code.

```rust
/* This is the entry point of the 
   application.
*/
fn main() {
    // Display a message to the user
    println!("Hello, world!");

    // my favorite color
    let c = "black"; // avoid

    let my_favorite_color = "black";
}
```

## Activity 1: Functions

In this activity, you'll learn how to define and use functions in Rust. Functions are a fundamental building block of Rust programming, allowing you to encapsulate and reuse code.

### Your Task

Before we dive into the details, here's a challenge for you: Try to implement the functionality described below on your own. Once you've given it a shot, you can compare your implementation with the example provided to see how well you did.

### Steps
- Display First Name: Define a function called `display_first_name` that prints your first name to the console.

- Display Last Name: Define another function called `display_last_name` that prints your last name to the console.

- Call Functions: Call both functions from the `main` function to display your full name.

```rust
// Define a function to display the first name
fn display_first_name() {
    println!("My first name is Mohammed");
}

// Define a function to display the last name
fn display_last_name() {
    println!("My last name is Shajahan");
}

// Main function
fn main() {
    // Call the display_first_name function
    display_first_name();
    
    // Call the display_last_name function
    display_last_name();
}
```

### Output

```bash
My first name is Mohammed
My last name is Shajahan
```

### Try It Yourself

Take a moment to implement the activity on your own. Once you're ready, compile and run your code using Cargo, and observe the output. Then, compare your implementation with the provided example to see how closely they match.

## Basic Arithmetic Operations

In this section, we'll explore the basic arithmetic operations available in Rust. These operations allow you to perform mathematical calculations within your Rust programs.

```rust
fn main() {
    let x = 100 + 1;   // Addition (+)
    let y = 9 - 4;    // Subtraction (-)
    let z = 5 * 8;   // Multiplication (*)
    let a = 20 / 5; // Division (/)
    let b = 10 % 3;// Remainder (Modulus) (%)
}
```

### Order of Operations

Just like in math class, Rust follows the order of operations (PEMDAS/BODMAS).

1. **P**arentheses
2. **E**xponents
3. **M**ultiplication and **D**ivision (from left to right)
4. **A**ddition and **S**ubtraction (from left to right)

For example:

```rust
let result = 10 + 5 * 2; // Rust does multiplication first, then addition
```

This would give `20`, because `5 * 2` is calculated first (giving `10`), and then `10` is added to `10`.

## Activity 2: Basic Arithmetic Operations

In this activity, you'll practice performing basic arithmetic operations in Rust. Follow the steps below to complete the activity

### Your Task

Try implementing the functionality described below on your own. Once you've completed it, compare your implementation with the example provided.

### Steps

- **Define Addition Function:** Create a function called `add_numbers` that takes two parameters `a` and `b` of type `i32` and returns their sum.

- **Display Result:** Define a function called `display_result` that takes a single parameter `result` of type `i32` and displays it to the console using the println macro with the `"{:?}"` token.

- **Call Functions:** In the `main` function, call the `add_numbers` function with two numbers of your choice and store the result. Then, call the `display_result` function to print the result.

```rust
// Define a function to add two numbers together
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Define a function to display the result
fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    // Call the add_numbers function to perform addition
    let result: i32 = add_numbers(10, 20);
    
    // Display the result
    display_result(result);
}
```
### Output

```
30
```

### Your Turn

Take some time to implement the activity on your own. Once you're done, compile and run your code using Cargo. Compare your output with the expected output provided above.

## Control Flow with if & else

In Rust, you can make decisions in your code using `if` and `else` statements. This is super handy when you want your program to do different things based on certain conditions.

### Basic Structure

```rust
if condition {
    // Do something if the condition is true
} else {
    // Do something else if the condition is false
}
```
Let's say we want to check if someone is old enough to purchase something

```rust
fn main() {
    let age = 15;
    if age >= 21 {
        println!("Ok to purchase");
    }else {
        println!("Cannot purchase");
    }
}
```

- We set `age` to `15`.
- The `if` statement checks if age is `21` or older.
- If it is, it prints `"Ok to purchase"`.
- If not, it prints `"Cannot purchase"`.

## Activity 3.1: Flow Control using if..else

In this activity, you'll practice using the `if..else` statement in Rust to display a message based on the value of a boolean variable.

### Your Task

Your task is to create a Rust program that displays a message based on the value of a boolean variable.
Steps

- **Define the Boolean Variable:** Create a boolean variable named `my_bool` and set it to either `true` or `false`.
- **Check the Value:** Use an `if..else` block to check the value of the `my_bool` variable. 
- **Display Message:** If `my_bool` is `true`, display "Hello". If it's `false`, display "Goodbye".

```rust
fn main() {
    // Define the boolean variable
    let my_bool = true;

    // Check the value and display message
    if my_bool {
        println!("Hello");
    } else {
        println!("Goodbye");
    }
}
```

### Output

```
Hello
```

### Try It Yourself

Implement the activity on your own. Once you're done, compile and run your code using Cargo to see if it produces the expected output.

## Activity 3.2: Flow Control using if..else if..else

In this activity, you'll use the `if..else if..else` statement in Rust to determine which message to display based on the value of a variable.

### Your Task

Your task is to create a Rust program that displays >5, <5, or =5 based on the value of a variable.
Steps

- **Define the Variable:** Create a variable named `x` and set it to any integer value.
- **Check the Value:** Use an `if..else if..else` block to check the value of the variable `x`.
- **Display Message:** Display `>5` if `x` is greater than 5, `<5` if `x` is less than 5, and `=5` if `x` is equal to 5.

```rust
fn main() {
    let x = 7;

    if x > 5 {
        println!(">5");
    } else if x < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
```

### Output

```
>5
```

### Try It Yourself

Implement the activity on your own. Once you're done, compile and run your code using Cargo to see if it produces the expected output.

