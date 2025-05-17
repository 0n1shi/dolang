# Dolang

🚧 THIS PROJECT IS STILL EXPERIMENTAL AND UNDER DEVELOPMENT 🚧

```dolang
let data = read_file("users.csv")

let rows =
  data
  |> split("\n")
  |> filter(not_empty)
  |> map(split(","))
  |> tail # remove header

let ids = rows |> map(first)
let names = rows |> map(second)

println(ids)
println(names)
```

## 🧭 Core Philosophy

> “Learn how to think, not just how to write” — a functional language with simple and consistent syntax to support that goal.

## 🍵 Preface: The “Do” in Dolang

> In Dolang, the prefix "Do" is inspired by the Japanese concept of 道 (dō), found in disciplines such as 茶道 (Sadō, the Way of Tea), 柔道 (Jūdō, the Way of Softness), and 弓道 (Kyūdō, the Way of the Bow). These traditions share a common principle: mastery through deliberate practice, refinement of form, and respect for process. Dolang aspires to follow the same path — not just as a programming language, but as a discipline that cultivates clarity of thought. By stripping away ambiguity and emphasizing consistency, Dolang encourages programmers to engage with code as a form of mindful craftsmanship. Its syntax is not designed to be clever, but to be clear, composed, and intentional — just like a well-practiced kata.

In this way, Dolang is more than a tool; it is a way.

## ✳️ Pillars of Language Design

1. Consistent and Explicit Syntax
   - Avoid ambiguous or multiple ways to express the same logic.
   - Minimize the number of exceptions that users need to learn.
2. Minimized Learning Cost
   - Intuitive and readable syntax without relying on symbolic tricks or shorthand.
   - Even beginners should understand not just how to write, but why.
3. Functional Programming Principles
   - Immutability by default; functions as first-class citizens.
   - Encourage data transformation using higher-order functions like map, filter, and reduce.
4. Disciplined Use of Syntactic Sugar
   - Syntactic sugar is only introduced when it provides clear, significant value.
   - Clarity is prioritized over convenience.

## 🏁 Goals

- Readable
- Memorable
- Thinkable

Dolang proposes a new kind of functional language: one that prioritizes discipline and simplicity, making functional thinking more approachable for everyone.

## ✍️ Notes on Future Extensions

- DSLs and utility functions may be introduced to enhance expressiveness.
- Instead of adding syntax, improvements will favor libraries, documentation, or REPL support.

## 🧱 Syntax Rules

### 1. Function calls always require parentheses

```dolang
add(1, 2)
split(",", text)
```

- ✅ Allowed: map(split(","))
- ❌ Disallowed: map split "," (no whitespace-based calls)
- 💡 Rationale: Clear, unambiguous parsing and consistent syntax.

### 2. Pipeline operator (|>) for function chaining

```dolang
let result = data |> filter(not_empty) |> map(split(","))
```

- Encourages left-to-right data transformation.
- Easier to read than deeply nested function calls.

### 3. Functions are curried by default

```dolang
map(split(","))
```

- All functions are curried, allowing partial application naturally.
- Enables expressive composition without anonymous functions.

### 4. Variable binding with let only

```dolang
let x = 42
```

- All variables are immutable.
- No reassignment; state changes must produce new values.

### 5. Expression-oriented syntax

```dolang
let y = if x > 0 then x else -x
```

- Constructs like if, match, and for are expressions and return values.
- Side effects are minimized and isolated.

### 6. No syntactic sugar for prefix or implicit application

- Expressions like not empty or map split "," are not supported.
- 💡 Rationale: Avoids parsing ambiguity and lowers the learning curve.

## 🧪 Design Trade-offs and Choices

| Feature                         | Adopted | Rationale                                                             |
| ------------------------------- | ------- | --------------------------------------------------------------------- |
| Mandatory parentheses for calls | ✅      | Eliminates ambiguity and promotes consistency                         |
| Pipeline operator (\|>)         | ✅      |                                                                       |
| Curried functions               | ✅      | Encourages composability and reuse                                    |
| Whitespace-based function calls | ❌      | Increases ambiguity, not beginner-friendly                            |
| Function composition via .      | ❌      | Introduces complex precedence rules and overuse of symbolic operators |
| Syntactic sugar                 | ❌      | Often adds learning cost and reduces syntactic transparency           |
