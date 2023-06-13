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

#prop[
    任意の単項式順序は整礎である．
]

#prf[
    略．
]

== 先頭イデアル

以下，多項式環 $R$ の単項式順序 $prec.eq$ を固定する．

#def[
    多項式 $f in R$ を
    $
        f = sum_(mu in cal(M)_R) c_mu dot mu
    $
    と表すとき，$c_mu eq.not 0$ となる $mu in cal(M)_R$ 全体の集合を
    $
        "supp"_R f := {mu in cal(M)_R | c_mu eq.not 0}
    $
    と書き，$f$ の *台* (_support_) と呼ぶ．
    多項式の台は有限集合であることに注意する．
    $f$ の台の，$prec.eq$ に関する最大元 $mu$ を $prec.eq$ に関する $f$ の *先頭単項式* (_initial monomial_) と呼び，$"in"_prec.eq f$ と書く．
    $c_mu$ を $prec.eq$ に関する $f$ の *先頭項係数* (_initial coefficient_)，$c_mu dot mu$ を $prec.eq$ に関する $f$ の *先頭項* (_initial term_) と呼び，それぞれ $"inic"_prec.eq f, space "init"_prec.eq f$ と書く．
]

#def[
    多項式環 $R$ のイデアル $I$ に対し，イデアル
    $
        "in"_prec.eq I := angle.l "in"_prec.eq f | f in I angle.r
    $
    を $I$ の *先頭イデアル* (_initial ideal_) と呼ぶ．
]

#rem[
    $f_1, ..., f_n in I$ が $I$ を生成するとき，$"in"_prec.eq f_1, ..., "in"_prec.eq f_n in "in"_prec.eq I$ は $"in"_prec.eq I$ を生成するとは限らない．
]

#def[
    $R$ のイデアル $I$ の生成元 $f_1, ..., f_n in I$ が $I$ の *Gröbner 基底* であるとは，先頭単項式 $"in"_prec.eq f_1, ..., "in"_prec.eq f_n in "in"_prec.eq I$ が先頭イデアル $"in"_prec.eq I$ を生成することをいう．
]
