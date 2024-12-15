# Zenith Language Specification

Zenith is a modern, JIT-compiled programming language that draws 
inspiration from C, Go, Rust, and Zig. Its syntax and semantics aim to 
strike a balance between performance, simplicity, and safety while 
maintaining flexibility for system-level programming.

## General Syntax Rules

1. **Whitespace:** Whitespace is generally ignored except where 
necessary to separate tokens.
2. **Case Sensitivity:** All identifiers are case-sensitive.
3. **Statements:** Each statement ends with a semicolon ;.
4. **Blocks:** Use braces {} to group statements.
5. **Comments:**
   - Single-line comments start with //.
   - Multi-line comments are enclosed in /* ... */.

---

## Primitives and Types

### Built-in Types
Zenith provides the following built-in types:

- **Numeric Types:**
  - Signed integers: i8, i16, i32, i64, i128
  - Unsigned integers: u8, u16, u32, u64, u128
  - Floating-point: f32, f64
  - Special: isize, usize (pointer-sized integers)
- **Boolean:** bool (values: true, false)
- **Character:** char (Unicode scalar value, 4 bytes)
- **String:** str (immutable, UTF-8 encoded)

### Custom Types
Zenith allows creating custom types:

- **Enumerations:**
``zenith
enum Color {
    Red,
    Green,
    Blue,
}
``


- **Structures:**
```zenith
struct Point {
    x: i32,
    y: i32,
}
```


- **Unions:**
- 
```zenith
union Data {
    int_value: i32,
    float_value: f32,
}
```


---

## Variables

### Declarations
Variables are declared using the var keyword.

```zenith
var x: i32 = 10;
var y = 20; // Type inferred as i32
```


### Mutability
By default, variables are immutable. Use mut to make them mutable.

```zenith
var mut counter: i32 = 0;
counter = counter + 1;
```


---

## Constants

Constants are declared using the const keyword. They must be initialized
 at compile time.
 
```zenith
const PI: f64 = 3.14159;
```


---

## Functions

Functions are declared using the fn keyword.

```zenith
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```


### Void Functions
Void functions do not return a value.

```zenith
fn log_message(message: str) {
    // Implementation
}
```


### Inline Functions
Functions can be inlined using the inline keyword.

```zenith
inline fn square(x: i32) -> i32 {
    return x * x;
}
```


---

## Control Flow

### If-Else

```zenith
if condition {
    // Code
} else if another_condition {
    // Code
} else {
    // Code
}
```


### Loops

- **While Loop:**

```zenith
while condition {
    // Code
}
```



- **For Loop:**

```zenith
for i in 0..10 {
    // Code
}
```


- **Infinite Loop:**

```zenith
loop {
    // Code
}
```

### Match

A match statement is used for pattern matching.

```zenith
match value {
    1 => println("One"),
    2 => println("Two"),
    _ => println("Other"),
}
```


---

## Memory Management

### Pointers

Pointers are explicitly defined using the * syntax.

```zenith
var ptr: *i32 = &value;
```


### References

References are created using the & operator.

```zenith
var ref: &i32 = &value;
```


---

## Error Handling

### Result Type
Zenith uses a Result type for error handling.

```zenith
fn divide(a: i32, b: i32) -> Result<i32, str> {
    if b == 0 {
        return Err("Division by zero");
    }
    return Ok(a / b);
}
```


### Panic

The panic function aborts execution.

```zenith
panic("Something went wrong");
```


---

## Modules

Modules are declared using the mod keyword.

```zenith
mod math {
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
}
```


---

## Attributes

Attributes provide metadata and are declared using #[...] syntax.

```zenith
#[inline]
fn fast_function() {
    // Code
}
```


---

## Generics

Functions, structs, and enums can be parameterized with types using 
generics.

```zenith
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}
```


---

## Operators

### Arithmetic
+, -, *, /, %

### Comparison
==, !=, <, >, <=, >=

### Logical
&&, ||, !

### Bitwise
&, |, ^, ~, <<, >>

---

## Macros

Macros in Zenith allow metaprogramming by generating code at compile 
time. Macros are defined using the macro keyword.

### Macro Definition
A macro definition starts with the macro keyword, followed by the macro 
name, parameters, and a block of code.

```zenith
macro repeat(n: i32, code: str) {
    for i in 0..n {
        code;
    }
}
```


### Macro Invocation
Macros are invoked with the @ symbol.

```zenith
@repeat(3, println("Hello, World!"));
```


### Inline Macros
Macros can also be defined inline using the #[] syntax for small 
snippets.

```zenith
#[repeat(n: i32, code: str) => for i in 0..n { code; }]
@repeat(5, println("Inline macro example"));
```


### Compile-Time Constants
Macros can compute constants at compile time.

```zenith
macro square_const(x: i32) -> i32 {
    return x * x;
}

const SQUARE_4: i32 = @square_const(4);
```


---

## Comments on Design
Zenith aims for:
1. Explicitness: Clear and concise syntax with minimal hidden behaviors.
2. Safety: Default immutability, explicit memory management.
3. Performance: JIT compilation and explicit control over resources.
