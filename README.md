```
cargo run
./replace
# manually remove -h, --help, -V and --version flags
# fair bit of manual changes to the zsh script
```

## Installation

### bash

```
source completions/zig.bash-completion
```

```
mv completions/_zig <path present in $fpath>
compinit -Uz _zig
```

**TODO**: Automate more of the generation or clean up the generated scripts.
