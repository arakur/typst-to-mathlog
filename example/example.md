<!-- #import "../style/mathlog_style.typ": * -->

# Gröbner 基底

## 単項式順序

$K$を体，$R=K\left[X_{1},…,X_{n}\right]$を$K$-上$n$変数多項式環とする．$R$の単項式全体の集合を$\mathcal{M}_{R}$とおく．$\mathcal{M}_{R}$は乗法に関して可換モノイドをなす．

&&&def 単項式順序
多項式環$R$の**単項式順序**(*monomial order*) とは，$\mathcal{M}_{R}$上の全順序$≼$であって，任意の$\mu,\mu',\nu∈\mathcal{M}_{R}$に対して以下を満たすもののことである：
1. $1≼\mu$;
2. $\mu≼\mu'⟹\mu\cdot\nu≼\mu'\cdot\nu$.
&&&

&&&prop
任意の単項式順序は整礎である．
&&&

&&&prf
略．
&&&

## 先頭イデアル

以下，多項式環$R$の単項式順序$≼$を固定する．

&&&def
多項式$f∈R$を
\begin{align*}
f=∑_{\mu∈\mathcal{M}_{R}}c_{\mu}\cdot\mu
\end{align*}
と表すとき，$c_{\mu}≠0$となる$\mu∈\mathcal{M}_{R}$全体の集合を
\begin{align*}
\mathrm{supp}_{R}f≔\left\{\mu∈\mathcal{M}_{R}|c_{\mu}≠0\right\}
\end{align*}
と書き，$f$の**台**(*support*) と呼ぶ．多項式の台は有限集合であることに注意する．$f$の台の，$≼$に関する最大元$\mu$を$≼$に関する$f$の**先頭単項式**(*initial monomial*) と呼び，$\mathrm{in}_{≼}f$と書く．$c_{\mu}$を$≼$に関する$f$の**先頭項係数**(*initial coefficient*)，$c_{\mu}\cdot\mu$を$≼$に関する$f$の**先頭項**(*initial term*) と呼び，それぞれ$\mathrm{inic}_{≼}f,\mathrm{init}_{≼}f$と書く．
&&&

&&&def
多項式環$R$のイデアル$I$に対し，イデアル
\begin{align*}
\mathrm{in}_{≼}I≔\langle\mathrm{in}_{≼}f|f∈I\rangle
\end{align*}
を$I$の**先頭イデアル**(*initial ideal*) と呼ぶ．
&&&

&&&rem
$f_{1},…,f_{n}∈I$が$I$を生成するとき，$\mathrm{in}_{≼}f_{1},…,\mathrm{in}_{≼}f_{n}∈\mathrm{in}_{≼}I$は$\mathrm{in}_{≼}I$を生成するとは限らない．
&&&

&&&def
$R$のイデアル$I$の生成元$f_{1},…,f_{n}∈I$が$I$の**Gröbner 基底**であるとは，先頭単項式$\mathrm{in}_{≼}f_{1},…,\mathrm{in}_{≼}f_{n}∈\mathrm{in}_{≼}I$が先頭イデアル$\mathrm{in}_{≼}I$を生成することをいう．
&&&
