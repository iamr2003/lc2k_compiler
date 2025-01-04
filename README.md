# LC2K Compiler

This is a simple compiler and tooling to target the [LC2K Instruction Set](https://eecs370.github.io/project_1_spec/#2-lc-2k-instruction-set-architecture).

Within this compiler, some additional conventions are imposed upon the instruction set:
- R0 is always 0
- R1 is the return register
- R5, R6 are stratch registers
- R7 stores the top of the stack

## Cimple Language
Currently, this compiler works on theC-like Cimple language grammar, defined in ``src/cimple.lalrpop``. 

This very simple language only supports the following operations:
- Addition expressions
- Variable assignment(In progress)
- Conditional(if/else) statements(TODO)
<!-- - Loops(while/for) -->
<!-- - Functions -->

### Examples

**Basic Expressions:**
#### **`in.cimple`**
```
22 + 33 + 24;
```
#### **`out.as`**
```
         lw      0       1       val_22  
         add     1       0       6       
         lw      0       1       val_33  
         add     1       6       1       
         add     1       0       6       
         lw      0       1       val_24  
         add     1       6       1       
end      halt    
val_22   .fill   22      
val_33   .fill   33      
val_24   .fill   24      
```


## (Goal) LLVM
In the future, making this an LLVM compiler will allow it to target far more languages.
