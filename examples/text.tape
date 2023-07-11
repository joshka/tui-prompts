# charm/vhs script to create a gif of the text example
# run with `vhs text.tape` then `vhs publish text.gif`
# for docs run `vhs new docs.tape`

Output "target/text.gif"
Require "cargo"

Set Height 210
Set Width 600
Set Shell zsh

Type "cargo -q run --example=text"
Enter
Sleep 2s
Type "asdf"
Enter
Sleep 2s
Type "password"
Enter
Sleep 2s
