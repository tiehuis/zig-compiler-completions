```
cargo run
./replace
# manually remove -h, --help, -V and --version flags
# fair bit of manual changes to the zsh script
```

**TODO**: Automate more of the generation or clean up the generated scripts.

## Installation

### bash

```
source completions/zig.bash-completion
```

### zsh

#### manual

```
mv completions/_zig <path present in $fpath>
compinit -Uz _zig
```

#### zplug

Add the following to your zsh config.

```
zplug "tiheuis/zig-compiler-completions", use:"completions/_zig", defer:2
```
