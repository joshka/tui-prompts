# charm/vhs script to create a gif of the text example
# run with `vhs text.tape` then `vhs publish text.gif`
# for docs run `vhs new docs.tape`

Output "examples/text.gif"
Require "cargo"

# Set Padding 10
Set Height 260
Set Width 600
Set Shell zsh

Type "cargo -q run --example text"
Enter
Sleep 2s
Type "asdf <enter>"
Enter
Sleep 2s
Type "cargo -q run --example text"
Enter
Sleep 2s
Type "asdf <esc>"
Sleep 2s
Escape
Sleep 2s
Type "cargo -q run --example text"
Enter
Sleep 2s
Type "long text that wraps around the screen"
Sleep 5s
