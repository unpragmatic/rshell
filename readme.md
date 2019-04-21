# RShell
A shell designed for speed and productivity. Made for a modern workflow and technologies.


## RShell Language
```
import system::directory;

let current_directory = !(pwd);
if current_directory == directory::from_relative("~") {
    println("Hello World");
}
```

### Grammer
```
PROGRAM := STATEMENT*
STATEMENT := SHELL_SCRIPT_COMMAND | SHELL_BUILTIN | PATH_PROGRAM
SHELL_SCRIPT_COMMAND := VARIABLE_ASSIGNMENT | EXPRESSION
VARIABLE_ASSIGNMENT := "let" VARIABLE_NAME "=" EXPRESSION
VARiABLE_NAME := /\p{L}\w/
EXPRESSION := STRING | NUMBER | FUNCTION | IF_EXPRESSION | LOOP_EXPRESSION
STRING := "\"" /.*/ "\""
NUMBER := /\d/
FUNCTION := "(" [VARIABLE_NAME*] ")" "->" "{" FUNCTION_BODY "}"
FUNCTION_BODY := [STATEMENT*]

TODO: think about function body. Should it have different behaviour
 from standard cmd interaction? My gut feeling is yes, but maybe 
 its more elegant the other way.
```
```
module system {

    module directory {

        let from_relative = (dir: string) -> string {
            // magic
            return result;
        }
    }
}
```

```
/lib/system/module.rsh
/lib/system/directory/module.rsh
```