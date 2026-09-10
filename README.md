My solution to (https://codingchallenges.fyi)[coding challenge] "build your own text editor".

## Requirements
Have Rust and Cargo installed.

Install using
```bash
git clone https://github.com/toewl-devv/tccvim
```

Then run
```bash
cd tccvim
cargo install --path .
```

To run the program, you can run
```bash
rim filename.txt
```
(or any other file) from any directory.

## Usage
In normal mode:

Movement: `hjkl`

Save: `w`

Quit: `q`

Toggle normal (0) and edit (1) modes: `Esc`

In edit mode:

Type as usual
