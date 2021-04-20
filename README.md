FunForth 
========

Language experiment involving concatenation, stacks and arrays, inspired by APL and Forth.

Built-in Words
--------------

### IO

 * `_` Print the top stack value.

### Control flow

 * `?` Conditionally evaluate the next code based on the truthiness of the top stack element. If falsy, continue from next `->`.
 * `→` Marker to identify branch end.

### Boolean operators

 * `⋀` Logical _and_ between top two stack elements. If each element has multiple values, perform element-wise _and_.
 * `⋁` Logical _or_ between top two stack elements. If each element has multiple values, perform element-wise _or.

All boolean operators repeat the top stack element, so `[ 1 0 1 ] 1 v` evaluates to `[ 1 1 1 ]`.

### Equality

 * `=` Test value-wise equality between top two stack elements.

### Mathematical operators

 * `+` Value-wise addition between top two stack elements.
 * `-` Value-wise subtraction between top two stack elements.
 * `*` Value-wise multiplication between top two stack elements.
 * `/` Value-wise division between top two stack elements.

All boolean operators repeat the top stack element, so `[ 1 2 3 ] 1 +` evaluates to `[ 2 3 4 ]`.

### In-element manipulation

 * `,` Concatenate top stack element to the following stack element.

### Stack manipulation

 * `▶` Duplicate top stack element.
 * `◀` Pop top stack element.
 * `◆` Swap the top two stack elements.
 * `▮` Clear the stack.
 * `▯` Clear all but the top stack element.

### Definitions
 * `:` Start word definition.
 * `;` End word definition.
 * `[` Start element definition. If element only contains a single value, the brackets can be omitted.
 * `]` End element definition.

