# charm/vhs script to create a gif of the text example
# run with `vhs text.tape` then `vhs publish text.gif`
# for docs run `vhs new docs.tape`

Output "target/text.gif"

Set Height 240
Set Width 600

Hide
Type "cargo build --example=text"
Enter
Type "cargo -q run --example=text"
Enter
Sleep 0.5s
Show
Sleep 2s
Type "ferris"
Sleep 0.5s
Enter
Sleep 2s
Type "password"
Sleep 0.5s
Enter
Sleep 2s
Type "a secret message"
Sleep 0.5s
Enter
Sleep 0.5s
Hide
Sleep 2s
