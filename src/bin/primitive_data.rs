/*
In Rust, primitive types are the most basic types that the language provides.
They are built into the language and do not require the use of any external libraries.
Below is a list of the common primitive types in Rust, categorized into different groups:

Scalar Types

Integer Types:
i8: Signed 8-bit integer

i16: Signed 16-bit integer

i32: Signed 32-bit integer

i64: Signed 64-bit integer

i128: Signed 128-bit integer

isize: Signed integer with size dependent on the architecture (32-bit or 64-bit)

u8: Unsigned 8-bit integer

u16: Unsigned 16-bit integer

u32: Unsigned 32-bit integer

u64: Unsigned 64-bit integer

u128: Unsigned 128-bit integer

usize: Unsigned integer with size dependent on the architecture (32-bit or 64-bit)

Length:
Length	Signed	Unsigned
8-bit   i8  	u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128   	u128
arch	isize	usize

Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive,
where n is the number of bits that variant uses.
So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127.
Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1,
which equals 0 to 255.

Floating-Point Types:
f32: 32-bit floating-point number
f64: 64-bit floating-point number (the default for floating-point numbers in Rust)

Boolean Type:
bool: Represents a boolean value, either true or false

Character Type:
char: Represents a single Unicode scalar value (a character),
and it must be enclosed insingle quotes (e.g., 'a', '😊')

Compound Types
Tuples:
A tuple is a fixed-size collection of values of potentially different types.
For example, (i32, f64, char) is a tuple with three elements of types
i32, f64, and char respectively.

Arrays:
An array is a fixed-size collection of values of the same type.
The length of the array must be known at compile time.
For example, [i32; 3] is an array of three i32 elements.

Other
String Slices:
&str: Represents a string slice, which is a reference to a sequence of UTF-8 encoded characters
within a string. It is immutable.

Function pointers:
Function pointers not types explicitly mentioned as primitives,
but they fit into the built-in/ intrinsic types.
An example is fn(), which signifies a function that takes no
arguments and returns (), the unit type.

The Unit Type
(): Represents an empty tuple, also known as the unit type.
It has exactly one possible value, which is also written ().
It is used as a return type for functions that do not return any other value.

Each of these types serves specific roles in programming with Rust,
allowing for efficiency, safety, and expressive code.
The Rust standard library builds upon these primitive types to offer more
complex data structures and functionality.
*/
