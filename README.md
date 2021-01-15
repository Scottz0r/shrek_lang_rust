# SHREK Lang

<img src="https://media.giphy.com/media/10Ug6rDDuG3YoU/giphy.gif" />

Are you an all star? Do you need a programming language that helps you get your game on? Do you need a language that will let you go play?

SHREK is a programming language that only uses the characters "SHREK!" SHREK is stack based, where commands manipulate the top values on the stack.

SHREK is written in Rust and is built with cargo. To execute SHREK scripts, run the shrek interpreter with the script path as the first argument.

## Examples

Printing 0, 1, 2 is easy as typing SHREK a few times... in slightly different ways.

```
SRRR # Counter value, set to 3

!R!
S # 0 value
SRE # Push 1/output, call func

R # Bump 0 to 1
SRE # Push 1/output, call func

R # Bump 1 to 2
SRE # Push 1/output, call func

H # Pop stack

# Subtract 1 from counter
SR # Push 1 to stack
SRRRE # Call subtract func (3) to {1} - {0}

SRK!E! # Jump if counter 0
SK!R! # Jump to !R!
!E!
S # Push 0 for exit code
```

Input and output is ezpz. The below will read a line from the user and rewrite it.

```
SE # Call input function

!S!
SRE # Call output function
H # Pop character from stack
SRK!H! # Jump to end if value is 0
SK!S! # Jump to !S!
!H!
```

## Reading this document

`{<num>}` will be used to represent an item on the stack *before* a command is executed. `{0}` represents the value on the top of the stack. The number will count up, so `{1}` represents the value just below the top of the stack.

Some commands use the top of the stack to determine sub-functionality. For example, the `jump` command uses `{0}` to determine the jump type. Some jump types will inspect the value after `{0}` (written as `{1}`) to conditionally jump. The numbers will always be based on the state of the stack before the command is executed.

`!!` will be used to represent a label.

## Syntax

|Letter|Command|Description|
|------|-------|-----------|
|S|push0|Pushes 0 to the top of the stack|
|H|pop|Pops the top of the stack|
|R|bump|Adds one to the top of the stack|
|E|func|Calls a function based on the value at `{0}`|
|K|jump|Jump to `!!` based on the value at `{0}`|
|!|label|Used to define a label|
|#|comment|Python-like comment. The `#` to the end of the line will be a comment (and can contain any character)

## Labels

Labels are defined with an opening and closing `!` character. Labels can only use the letters in "SHREK". For example `!S!` will define the label `S`.

## Jump Command

When jumping, the value at `{0}` defines what type of jump to perform. A jump command must be followed by a label. If the target label is not defined, the program will terminate upon the jump.

The jump command will remove `{0}` from the stack.

|`{0}` Value|Command|Description|
|-----------|-------|-----------|
|0|jump|Always jump to label|
|1|jump 0|Jump to label if `{1}` == 0|
|2|jump neg|Jump to label if `{1}` < 0|

## Func Command

The func command uses the top of the stack to determine which function to call. Functions are mapped in a function table. See the [C Extension API](#c-extension-api) for writing and registering custom functions.

The func command will remove `{0}` from the stack.

SHREK comes with the following built-in commands.

### 0. Input
Read string from stdin and place on stack. The string will be added to the stack in reverse order, so popping the stack will return the string in the correct order. Strings will be null terminated.

### 1. Output
|Write `{1}` to stdout. `{1}` will not be popped by this function

### 2. Add

Add `{2}` to `{1}`. `{1}` and `{2}` will be popped, and the result will be placed on the top of the stack.

### 3. Subtract
Subtract `{2}` from `{1}`. `{1}` and `{2}` will be popped, and the result will be placed on the top of the stack.

### 4. Multiply
Multiply `{2}` by `{1}`. `{1}` and `{2}` will be popped, and the result will be placed on the top of the stack.

### 5. Divide (integer division)
Divide `{2}` by `{1}`. `{1}` and `{2}` will be popped, and the result will be placed on the top of the stack.

### 6. Mod
Get the remainder of dividing `{2}` by `{1}`. `{1}` and `{2}` will be popped, and the result will be placed on the top of the stack.

### 7. Double

Double the value of `{1}`.

### 8. Negate

Multiple `{1}` by -1.

### 9. Square

Square `{1}`

### 10. Clone

Put a copy of `{1}` on the top of the stack

## Optimization

"Ugh, this language is slow," is what you are thinking. But not to fear. The interperter will detect and optimize constant values. Long chains of push and bumps will be squashed into a single push_constant command in the op code. The optimizer will also optimize arithmetic on constant values.

<img src="https://media.giphy.com/media/bLFQRUZGisPJe/giphy.gif">
