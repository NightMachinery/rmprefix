# Usage

`rmprefix` removes the given prefix from its input if present. The input is split using the given separator, by default newlines.

Usage: `rmprefix <prefix> [<input-separator>=\n] [<output-separator>=\n]`

Set the separators to \x00 for the NUL character.

## Examples

```
echo "The jungles are green.
Some people sing.
The world is not flat.
There are two ways." | rmprefix The
# =>
# jungles are green.
#Some people sing.
# world is not flat.
#re are two ways.

echo "123456789" "23456789" "12345" "Hi" "" "Ocean" | rmprefix 123 ' ' '|||'                                             
# => 456789|||23456789|||45|||Hi||||||Ocean
```

# Installation

```
cargo install --git https://github.com/NightMachinary/rmprefix
```
