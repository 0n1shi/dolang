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
}

pub const BUILTIN_FUNCTIONS: &[BuiltinFunc] = &[
    // IO
    BuiltinFunc {
        name: "print",
        func: print,
        args_len: 1,
    },
    BuiltinFunc {
        name: "println",
        func: println,
        args_len: 1,
    },
    // File
    BuiltinFunc {
        name: "read_file",
        func: read_file,
        args_len: 1,
    },
    // List
    BuiltinFunc {
        name: "map",
        func: map,
        args_len: 2,
    },
    BuiltinFunc {
        name: "filter",
        func: filter,
        args_len: 2,
    },
    BuiltinFunc {
        name: "append",
        func: append,
        args_len: 2,
    },
    BuiltinFunc {
        name: "first",
        func: first,
        args_len: 1,
    },
    BuiltinFunc {
        name: "second",
        func: second,
        args_len: 1,
    },
    BuiltinFunc {
        name: "third",
        func: third,
        args_len: 1,
    },
    BuiltinFunc {
        name: "tail",
        func: tail,
        args_len: 1,
    },
    BuiltinFunc {
        name: "last",
        func: last,
        args_len: 1,
    },
    BuiltinFunc {
        name: "sum",
        func: sum,
        args_len: 1,
    },
    // String
    BuiltinFunc {
        name: "int",
        func: int,
        args_len: 1,
    },
    BuiltinFunc {
        name: "split",
        func: split,
        args_len: 2,
    },
    // Number
    BuiltinFunc {
        name: "str",
        func: str,
        args_len: 1,
    },
    // etc
    BuiltinFunc {
        name: "len",
        func: len,
        args_len: 1,
    },
    BuiltinFunc {
        name: "empty",
        func: empty,
        args_len: 1,
    },
    BuiltinFunc {
        name: "not_empty",
        func: not_empty,
        args_len: 1,
    },
];
