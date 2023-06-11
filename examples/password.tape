# charm/vhs script to create a gif of the password example
# run with vhs password.tape then vhs publish password.gif

Output "password.gif"
Require "cargo"

Set Padding 10
Set Height 110
Set Width 600
Set Shell zsh

Type "cargo -q run --example password"
Enter
Sleep 500ms
Type "asdf"
Enter
Sleep 5s
Escape
