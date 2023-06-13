# Write Mathlog markup with Typst(WIP)

[Mathlog](https://mathlog.info/) のマークアップを [Typst](https://typst.app/) で書いて変換するツールです．

A tool to write [Mathlog](https://mathlog.info/) markup with [Typst](https://typst.app/).

## Example

Typst source:

```typst
// set mathlog style

#import "../style/mathlog_style.typ": *


// 

= Gröbner 基底

== 単項式順序

$K$ を体，$R = K[X_1, ..., X_n]$ を $K$-上 $n$ 変数多項式環とする．
$R$ の単項式全体の集合を $cal(M)_R$ とおく．
$cal(M)_R$ は乗法に関して可換モノイドをなす．

#def(title: "単項式順序")[
    多項式環 $R$ の *単項式順序* (_monomial order_) とは，$cal(M)_R$ 上の全順序 $prec.eq$ であって，任意の $mu, mu', nu in cal(M)_R$ に対して以下を満たすもののことである：
    
    1. $1 prec.eq mu$;
    2. $mu prec.eq mu' ==> mu dot nu prec.eq mu' dot nu$.
]
```

Mathlog result:

```mathlog
<!-- #import "../style/mathlog_style.typ": * -->

# Gröbner 基底

## 単項式順序

$K$を体，$R=K\left[X_{1},…,X_{n}\right]$を$K$-上$n$変数多項式環とする．$R$の単項式全体の集合を$\mathcal{M}_{R}$とおく．$\mathcal{M}_{R}$は乗法に関して可換モノイドをなす．

&&&def 単項式順序
多項式環$R$の**単項式順序**(*monomial order*) とは，$\mathcal{M}_{R}$上の全順序$≼$であって，任意の$\mu,\mu',\nu∈\mathcal{M}_{R}$に対して以下を満たすもののことである：
1. $1≼\mu$;
2. $\mu≼\mu'⟹\mu\cdot\nu≼\mu'\cdot\nu$.
&&&
```

## Usage

Install an asset on GitHub Releases(only for Windows) or build from source (requires cargo 1.70.0).

`style/mathlog_style.typ` is a style file for Mathlog-like environments and styles.
You can use it by

```typst
#import "style/mathlog_style.typ": *
```

in Typst source.

After you have written Typst source, then run `bin/typst-to-mathlog.exe` with the following arguments:

```sh
typst-to-mathlog <input> <output>
```

The directory `dictionary` includes a dictionary file `dictionary.json` to convert commands in Typst source to ones in TeX
`dictionary/dictionary_unicode.json` includes all characters which can be written in Typst, but it converts all to unicode characters.
`dictionary/dictionary_patch.json` is a patch file for this, which rewrite some commands into TeX native commands.
One can change the dictionary file by the following way:

1. rewrite `dictionary/dictionary_patch.json`,
2. run `make_dictionary.py`.

## TODO

- [ ] Support all environments
- [ ] Support links
- [ ] Support labels/refs
- [ ] Support tables
- [ ] Support images
- [ ] Support all math commands

## License

MIT License
