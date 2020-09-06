# DEPRECATED

Use https://github.com/NightMachinary/prefixer instead. I'll continue to support bugfixes for `rmprefix`, as it's a very lightweight command and I still use it in my  scripts, but `prefixer` is better in virtually every way.

# Usage

`rmprefix` removes the given prefix from its input if present. The input is split using the given separator, by default newlines.

Usage: `rmprefix <prefix> [<input-separator>=\n] [<output-separator>=\n]`

Set the separators to `\x00` for the NUL character.

`rmprefix` can also work as a search and replace tool, with this neat trick:

`rmprefix '' SEARCH_FOR_ME REPLACE_WITH_ME`

## Examples

```
echo "The jungles are green.
Some people sing.
The world is not flat.
There are two ways." | rmprefix The
```
 
> jungles are green.
>
>Some people sing.
>
> world is not flat.
>
>re are two ways.

```
echo "123456789" "23456789" "12345" "Hi" "" "Ocean" | rmprefix 123 ' ' '|||'                                             
```

> 456789|||23456789|||45|||Hi||||||Ocean

```
echo "Butterflies are insects in the macrolepidopteran clade Rhopalocera from the order Lepidoptera, which also includes moths. Adult butterflies have large, often brightly coloured wings, and conspicuous, fluttering flight. The group comprises the large superfamily Papilionoidea, which contains at least one former group, the skippers (formerly the superfamily "Hesperioidea"), and the most recent analyses suggest it also contains the moth-butterflies (formerly the superfamily "Hedyloidea"). Butterfly fossils date to the Paleocene, about 56 million years ago." | rmprefix '' butter cheese
```

> Butterflies are insects in the macrolepidopteran clade Rhopalocera from the order Lepidoptera, which also includes moths. Adult cheeseflies have large, often brightly coloured wings, and conspicuous, fluttering flight. The group comprises the large superfamily Papilionoidea, which contains at least one former group, the skippers (formerly the superfamily Hesperioidea), and the most recent analyses suggest it also contains the moth-cheeseflies (formerly the superfamily Hedyloidea). Butterfly fossils date to the Paleocene, about 56 million years ago.

# Installation

```
cargo install --git https://github.com/NightMachinary/rmprefix
```

# Licenses

Dual-licensed under MIT and GPL v3 or later.
