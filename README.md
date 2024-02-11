# Programming Rust

## Chapter 2 : A Tour of Rust
Rust is an amazing systems level language that considers type safety as a primary factor. Thus, it is a fast and gorgeous language that cares a lot about memory safety, error handling and type systems. Chapter 2 is a basic introduction to Rust, where I implemented basic functions to realize the notion of this language. Inside the `mandelbrot` project, the Mandelbrot set is evaluated and visualized in a concurrent way using 8 threads. It is very efficient and easy to write a multi-threaded code in Rust.

## Chapter 3 : Basic Types
Inside Rust, there are very many internal types designed by the Type Class method. This allows even primitive types to have methods of their own, which is a charming feature of this language. We examine multiple types and useful built-in methods for each types. Finally, we appreciate the thorough type infer system of rust, where all the types are statically analyzed before compiling. This makes rust a very easy language to use, and there are as less bugs that may occur in compile time than any other language.

## Chapter 4 : Ownership
Rust has a special notion of handling memory, which is called ownership. Ownership means that a variable owns another variable, for example a vector owns its elements, and each element owns its attribute, and so on. This is very convenient when it comes to memory handling, as the relation of ownerships form a tree. If one node(variable) is freed from the memory, it is plausible to assume that the subtree of the ownership tree that has the root of that particular node is freed as well. This ownership causes some breakthroughs that have not been considered in other PLs. 

Note that this makes Rust kind of difficult for beginners, because `move` is somewhat not intuitive. Each value is moved, which means that the owner is modified. For example, if a variable is called as a parameter of some function, then the value is moved. This is anti-intuitive because if we iterate a vector and print all of the members in it using the for-loop, then the vector is now uninitialized. 