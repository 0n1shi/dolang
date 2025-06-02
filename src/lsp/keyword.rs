pub struct Keyword {
    pub name: &'static str,
    pub description: &'static str,
}

pub const KEYWORDS: &[Keyword] = &[
    Keyword {
        name: "let",
        description: "Declares a variable with a specified name and value.",
    },
    Keyword {
        name: "fn",
        description: "Defines a function with a specified name and parameters.",
    },
    Keyword {
        name: "if",
        description: "Conditional statement that executes code based on a condition.",
    },
    Keyword {
        name: "then",
        description: "Code block executed if the 'if' condition is true.",
    },
    Keyword {
        name: "else",
        description: "Alternative code block executed if the 'if' condition is false.",
    },
    Keyword {
        name: "in",
        description: "Checks if a value is present in a collection or range.",
    },
    Keyword {
        name: "is",
        description: "Checks if a value matches a specific type or condition.",
    },
    Keyword {
        name: "match",
        description: "Pattern matching construct for handling different cases.",
    },
    Keyword {
        name: "and",
        description: "Logical operator that returns true if both operands are true.",
    },
    Keyword {
        name: "or",
        description: "Logical operator that returns true if at least one operand is true.",
    Keyword {
        name: "not",
        description: "Logical operator that negates the truth value of an operand.",
    },
];
