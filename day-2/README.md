# 11-05-2023

## Common Programming Concepts

### Variables and Mutability

The difference between `mut` and shadow:

- `mut` allows to mutate the variables in same type.
- shadow allows to change the type of the variable.
- One thing cool about shadow is I don't have to rename a variable when the type is different, this is something JavaScript doesn't have.

```
let spaces = "   ";
let spaces = spaces.len();

let mut spaces = "   ";
spaces = spaces.len();
```

### Data Types
