# charm/vhs script to create a gif of the password example
# run with `vhs password.tape` then `vhs publish password.gif`
# for docs run `vhs new example.tape`

Output "examples/password.gif"
Require "cargo"

Set Height 260
Set Width 600
Set Shell zsh

Type "cargo -q run --example password"
Enter
Sleep 2s
Type "asdf"
Escape
Sleep 2s
Type "cargo -q run --example password"
Enter
Sleep 2s
Type "pass...."
Sleep 2s
Backspace 4
Type "word"
Enter
Sleep 5s
