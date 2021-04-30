FunForth 
========

Language experiment involving concatenation, stacks and arrays, inspired by APL and Forth.

Built-in Words
--------------

### IO

 * `_` Print the top stack value.

### Control flow

 * `if` Conditionally evaluate the next code based on the truthiness of the top stack element. If falsy, continue from next `->`.
 * `then` Marker to identify branch end.

### Boolean operators

 * `and` Logical _and_ between top two stack elements. If each element has multiple values, perform element-wise _and_.
 * `or` Logical _or_ between top two stack elements. If each element has multiple values, perform element-wise _or.

All boolean operators repeat the top stack element, so `[ 1 0 1 ] 1 v` evaluates to `[ 1 1 1 ]`.

### Equality

 * `eql` Test value-wise equality between top two stack elements.

### Arithmetic operators

 * `+` Value-wise addition between top two stack elements.
 * `-` Value-wise subtraction between top two stack elements.
 * `*` Value-wise multiplication between top two stack elements.
 * `/` Value-wise division between top two stack elements.

All arithmetic operators repeat the top stack element, so `[ 1 2 3 ] 1 +` evaluates to `[ 2 3 4 ]`.

### In-element manipulation

 * `cat` Concatenate top stack element to the following stack element.

### Stack manipulation

 * `dup` Duplicate top stack element.
 * `pop` Pop top stack element.
 * `swp` Swap the top two stack elements.
 * `clr` Clear the stack.
 * `clr1` Clear all but the top stack element.

### Definitions
 * `:` Start word definition.
 * `;` End word definition.
 * `[` Start element definition. If element only contains a single value, the brackets can be omitted.
 * `]` End element definition.

