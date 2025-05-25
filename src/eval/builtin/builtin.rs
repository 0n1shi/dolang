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
        description: "Prints a value to the standard output without a newline.",
    },
    BuiltinFunc {
        name: "println",
        func: println,
        args_len: 1,
        description: "Prints a value to the standard output with a newline.",
    },
    // File
    BuiltinFunc {
        name: "read_file",
        func: read_file,
        args_len: 1,
        description: "Reads the contents of a file and returns it as a string.",
    },
    // List
    BuiltinFunc {
        name: "map",
        func: map,
        args_len: 2,
        description: "Applies a function to each element of a list and returns a new list.",
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
        description: "Appends an element to a list and returns the new list.",
    },
    BuiltinFunc {
        name: "first",
        func: first,
        args_len: 1,
        description: "Returns the first element of a list.",
    },
    BuiltinFunc {
        name: "second",
        func: second,
        args_len: 1,
        description: "Returns the second element of a list.",
    },
    BuiltinFunc {
        name: "third",
        func: third,
        args_len: 1,
        description: "Returns the third element of a list.",
    },
    BuiltinFunc {
        name: "tail",
        func: tail,
        args_len: 1,
        description: "Returns a new list without the first element.",
    },
    BuiltinFunc {
        name: "last",
        func: last,
        args_len: 1,
        description: "Returns the last element of a list.",
    },
    BuiltinFunc {
        name: "sum",
        func: sum,
        args_len: 1,
        description: "Returns the sum of all elements in a list.",
    },
    // String
    BuiltinFunc {
        name: "int",
        func: int,
        args_len: 1,
        description: "Converts a string to an integer.",
    },
    BuiltinFunc {
        name: "split",
        func: split,
        args_len: 2,
        description: "Splits a string into a list of strings based on a delimiter.",
    },
    // Number
    BuiltinFunc {
        name: "str",
        func: str,
        args_len: 1,
        description: "Converts a number to a string.",
    },
    // etc
    BuiltinFunc {
        name: "len",
        func: len,
        args_len: 1,
        description: "Returns the length of a list or string.",
    },
    BuiltinFunc {
        name: "empty",
        func: empty,
        args_len: 1,
        description: "Checks if a list or string is empty.",
    },
    BuiltinFunc {
        name: "not_empty",
        func: not_empty,
        args_len: 1,
        description: "Checks if a list or string is not empty.",
    },
];
