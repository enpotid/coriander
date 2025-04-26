<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/40e752a2-c783-4216-9516-fe4147a233ff">
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/9688be3e-2097-46db-be59-eb6c5e4f77bc">
    <img alt="The Coriander Programming Language"
         src="https://github.com/user-attachments/assets/9688be3e-2097-46db-be59-eb6c5e4f77bc"
         width="50%">
  </picture>

[Website] | [Learn]
</div>

[Website]: https://enpotid.github.io/coriander
[Learn]: https://enpotid.github.io/coriander/learn

# Why Coriander?
It can use libraries without an OS... and it is very low-level!
# Grammar
- Declarations
```
ident() {}    // Function
Ident[] {}    // Enum
Ident{} {}    // Struct
```
- Function prefix (option xor)
```
~    // No ret
=    // No mangle
!    // No function setting
#    // Global function
>    // Entry point (~, =, !, #)
-    // Inline function (~, !) If this is used, other prefixes will be ignored
^    // Just jump (~, !)
```
- Keyword
```
let       // let idnet = x;
nec       // nec std;
index     // index std!!io!!println;
return    // return x;
match     // match x {}
```
