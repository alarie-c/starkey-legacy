# Values & Variables

Declare values with or without types
```
x = 10;                         ~ int
x :: int = 10;                  ~ int
```
```
y = 3.1415;                     ~ num
y :: num = 3.1415;              ~ num
```

```
g = 0;                          ~ int
g :: uint = 0;                  ~ uint
```

Declare values mutable with mut keyword
```
mut x :: int = 5;               ~ mut int
mut y :: str = "Hello";         ~ mut str
```

Mutate a variable
```
mut index :: 0;
index <- 1;                     ~ mutates `index` to 1
```

```
mut welcome :: str = "Hello, ";
name :: str = "John Doe";
welcome <- concat(welcome, name);
```

Attempting to mutate a value
```
version :: str = "1.0.1";
version <- "1.2.1"              ~ err: `version` is not a mutable value
```

---

# Functions & Procedures
Functions and procedures but be different from each other.
Functions cannot modify any values outside of their environment.
This means functions that do this must be annotated with "mut."

Basic function
```
func add_five(x :: int) -> int
{
    return x + 5;
}
```

Function that modifies values outside its environment
```
my_map :: std.HashMap[int, str] = std.HashMap.empty();

mut func new_entry(const value :: int, const key :: str)
{
    my_map.insert(value, key);
} 
```

Using "mut" keyword to declare a value changing
```
func print_student_name(mut student :: Student) 
{
    $ student.name <- "Jane Doe";
}
```

---

# General Philosophy

This is just C++ and Rust but WITHOUT memory safety

Everything is pass by value. References, pointers, addresses, whatever
dont exist in Starkey

That said, it doesn't mean Starkey isn't complex in it's type annotation

Things like mut, const, etc. must be used to denote HOW you're using data

This goes along with the philosophy that you shouldn't ever be modifying
data unless explicitly apparent.

A function like "print" could have a small part where it changes the data
that you forgot you added. Suddenly, you're stuck wondering why the output
doesn't match the code that you're writing. This is why things like "const"
are important.

```
func print(mut data :: Data)
{
    data.value <- 4;
    $ data;
}
```
> In this scenario, we KNOW that print changes the data given to it

The tags like cons and mut aren't just important for you, but also the compiler
The compiler needs to know if you've told it something can be mutated.
The compiler needs to know if you've told it that it's allowed to change values in a function parameter.