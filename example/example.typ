#import "../style.typ"
#import style: *

#show strong: set text(fill: strong_color)

// 

= 加群の Gröbner 基底

- (TODO: 加群項順序の説明)
- (TODO: 加群の Gröbner 基底の説明)
- (TODO: 消去定理の説明)

= Koszul 複体

#def[
    多項式環 $R = k[X_1, ..., X_n]$, その上の自由加群 $E = ⨁_(i=1)^l R bold(e)_i$ と $R$-加群準同型写像 $φ: E → R$ に対し，
    $
        K_j(φ) := ⋀^j E = ⨁_(i_1 < ... < i_j) bold(e)_(i_1) ∧ ... ∧ bold(e)_(i_j) R, \
        d_j: K_(j+1)(φ) → K_j(φ): bold(e)_(i_1) ∧ ... ∧ bold(e)_(i_(j+1)) ↦ ∑_(k=1)^(j+1) (-1)^(k+1) φ(bold(e)_k) bold(e)_(i_1) ∧ ... ∧ bold(e)_(i_(k-1)) ∧ bold(e)_(i_(k+1)) ∧ ... ∧ bold(e)_(i_(j+1))
    $
    と定めて得られる複体 $K_∙(φ)$ を $φ$ の *Koszul 複体* という．
]

これは確かに複体になる．
実際，
$
    &quad d_j (d_(j+1) (bold(e)_(i_1, ..., i_(j+2)))) \
    &= d_j (∑_(k=1)^(j+2) (-1)^(k+1) φ(bold(e)_k) bold(e)_(i_1) ∧ ... ∧ bold(e)_(i_(k-1)) ∧ bold(e)_(i_(k+1)) ∧ ... ∧ bold(e)_(i_(j+2))) \
    &= ∑_(k < k') (-1)^(k+k'+1) φ(bold(e)_k) φ(bold(e)_(k')) bold(e)_(i_1) ∧ ... ∧ bold(e)_(i_(k-1)) ∧ bold(e)_(i_(k+1)) ∧ ... ∧ bold(e)_(i_(k'-1)) ∧ bold(e)_(i_(k'+1)) ∧ ... ∧ bold(e)_(i_(j+2)) \
    &+ ∑_(k > k') (-1)^(k+k') φ(bold(e)_k) φ(bold(e)_(k')) bold(e)_(i_1) ∧ ... ∧ bold(e)_(i_(k-1)) ∧ bold(e)_(i_(k+1)) ∧ ... ∧ bold(e)_(i_(k'-1)) ∧ bold(e)_(i_(k'+1)) ∧ ... ∧ bold(e)_(i_(j+2)) \
    &= 0
$
である．

以下，空の外積を $bb(1) := ⋀ ∅ ∈ K_0(φ)$ と書く．

#ex[
    $n = 2$, $φ: E → R: bold(e)_1 ↦ X_1^2, bold(e)_2 ↦ X_1 X_2$ のとき，
    $
        K_0(φ) = bb(1) R, \
        K_1(φ) = E = bold(e)_1 R ⊕ bold(e)_2 R, \
        K_2(φ) = E ∧ E = bold(e)_1 ∧ bold(e)_2 R, \
        d_0: K_1(φ) → K_0(φ): bold(e)_1 ↦ X_1^2 bb(1), space bold(e)_2 ↦ X_1 X_2 bb(1), \
        d_1: K_2(φ) → K_1(φ): bold(e)_1 ∧ bold(e)_2 ↦ X_1^2 bold(e)_2 - X_1 X_2 bold(e)_1
    $
    より，Koszul 複体は以下のようになる：
]
