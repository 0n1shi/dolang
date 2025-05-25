use crate::eval::builtin::etc::{empty, len, not_empty};
use crate::eval::builtin::file::read_file;
use crate::eval::builtin::io::{print, println};
use crate::eval::builtin::list::{append, filter, first, last, map, second, sum, tail, third};
use crate::eval::builtin::num::str;
use crate::eval::builtin::str::{int, split};
use crate::eval::value::Value;

pub struct BuiltinFunc {
    pub name: &'static str,
    pub func: fn(Vec<Value>) -> Result<Value, String>,
    pub args_len: usize,
    pub description: &'static str,
}

pub const BUILTIN_FUNCTIONS: &[BuiltinFunc] = &[
    // IO
    BuiltinFunc {
        name: "print",
        func: print,
        args_len: 1,
        description: r#"Prints a value to the standard output without a newline.
This function is useful for debugging or displaying information.

Example:
        print("Hello, World!")"#,
    },
    BuiltinFunc {
        name: "println",
        func: println,
        args_len: 1,
        description: r#"Prints a value to the standard output with a newline.
This function is useful for displaying information in a readable format.

Example:
        println("Hello, World!")"#,
    },
    // File
    BuiltinFunc {
        name: "read_file",
        func: read_file,
        args_len: 1,
        description: r#"Reads the contents of a file and returns it as a string.
This function is useful for reading data from files.

Example:
        read_file("path/to/file.txt")"#,
    },
    // List
    BuiltinFunc {
        name: "map",
        func: map,
        args_len: 2,
        // description: "Applies a function to each element of a list and returns a new list.",
        description: r#"Applies a function to each element of a list and returns a new list.
This function is useful for transforming lists.

Example:
        map([1, 2, 3], fn(x) { x * 2 }) # [2, 4, 6]"#,
    },
    BuiltinFunc {
        name: "filter",
        func: filter,
        args_len: 2,
        description: "Filters elements of a list based on a predicate function.",
    },
    BuiltinFunc {
        name: "append",
        func: append,
        args_len: 2,
        // description: "Appends an element to a list and returns the new list.",
        description: r#"Appends an element to a list and returns the new list.
This function is useful for adding elements to lists.

Example:
        append([1, 2, 3], 4) # [1, 2, 3, 4]"#,
    },
    BuiltinFunc {
        name: "first",
        func: first,
        args_len: 1,
        description: r#"Returns the first element of a list.
This function is useful for accessing the first item in a list.

Example:
        first([1, 2, 3]) # 1 "#,
    },
    BuiltinFunc {
        name: "second",
        func: second,
        args_len: 1,
        description: r#"Returns the second element of a list.
This function is useful for accessing the second item in a list.

Example:
        second([1, 2, 3]) # 2 "#,
    },
    BuiltinFunc {
        name: "third",
        func: third,
        args_len: 1,
        description: r#"Returns the third element of a list.
This function is useful for accessing the third item in a list.

Example:
        third([1, 2, 3]) # 3 "#,
    },
    BuiltinFunc {
        name: "tail",
        func: tail,
        args_len: 1,
        description: r#"Returns a new list containing all elements except the first.
This function is useful for accessing the tail of a list.

Example:
        tail([1, 2, 3]) # [2, 3]"#,
    },
    BuiltinFunc {
        name: "last",
        func: last,
        args_len: 1,
        description: r#"Returns the last element of a list.
This function is useful for accessing the last item in a list.

Example:
        last([1, 2, 3]) # 3 "#,
    },
    BuiltinFunc {
        name: "sum",
        func: sum,
        args_len: 1,
        description: r#"Returns the sum of all elements in a list.
This function is useful for calculating the total of numeric lists.

Example:
        sum([1, 2, 3]) # 6 "#,
    },
    // String
    BuiltinFunc {
        name: "int",
        func: int,
        args_len: 1,
        description: r#"Converts a string to an integer.
This function is useful for parsing numeric strings.

Example:
        int("123") # 123 "#,
    },
    BuiltinFunc {
        name: "split",
        func: split,
        args_len: 2,
        description: r#"Splits a string into a list of substrings based on a delimiter.
This function is useful for breaking down strings into manageable parts.

Example:
        split("a,b,c", ",") # ["a", "b", "c"]"#,
    },
    // Number
    BuiltinFunc {
        name: "str",
        func: str,
        args_len: 1,
        description: r#"Converts a number to a string.
This function is useful for formatting numbers as strings.

Example:
        str(123) # "123""#,
    },
    // etc
    BuiltinFunc {
        name: "len",
        func: len,
        args_len: 1,
        description: r#"Returns the length of a list or string.
This function is useful for determining the size of collections.

Example:
        len([1, 2, 3]) # 3
        len("hello") # 5"#,
    },
    BuiltinFunc {
        name: "empty",
        func: empty,
        args_len: 1,
        description: r#"Checks if a list or string is empty.
This function is useful for checking if collections have no elements.

Example:
        empty([]) # true
        empty("hello") # false"#,
    },
    BuiltinFunc {
        name: "not_empty",
        func: not_empty,
        args_len: 1,
        description: r#"Checks if a list or string is not empty.
This function is useful for confirming that collections contain elements.

Example:
        not_empty([1, 2, 3]) # true
        not_empty("") # false"#,
    },
];
