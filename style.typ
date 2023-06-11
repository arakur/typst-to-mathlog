#let strong_color = red
#let def_color = rgb("11508e")
#let thm_color = rgb("17b0c4")
#let ex_color = rgb("d4bd77")
#let exc_color = black
#let rem_color = rgb("ffebee")
#let prf_color = black

// 

#show strong: set text(fill: strong_color)

#let env(counter, color, title, name, body) = block(
    stroke: (
        paint: color,
        thickness: 1pt,
    ),
    radius: 2pt,
    inset: 4pt,
    width: 100%,
    breakable: true
)[
    #counter.step()
    #text(fill: color, weight: "bold")[#name #counter.display()]
    #if title != none [
        text[ #title ]
    ]
    
    #body
]

#let def_counter = counter("def")
#let def(title: none, body) = env(def_counter, def_color, title, "定義", body)

#let thm_counter = counter("thm")
#let thm(title: none, body) = env(thm_counter, thm_color, title, "定理", body)
#let prop(title: none, body) = env(thm_counter, thm_color, title, "命題", body)
#let lem(title: none, body) = env(thm_counter, thm_color, title, "補題", body)

#let ex_counter = counter("ex")
#let ex(title: none, body) = env(ex_counter, ex_color, title, "例", body)

#let exc_counter = counter("exc")
#let exc(title: none, body) = env(exc_counter, exc_color, title, "問題", body)

#let rem(title: none, body) = block(
    fill: rem_color,
    radius: 2pt,
    inset: 4pt,
    width: 100%,
    breakable: true
)[
    #text(fill: strong_color, weight: "bold")[注意]
    #if title != none [
        text[ #title ]
    ]
    
    #body
]

#let prf(title: none, body) = block(
    stroke: (
        "left": (
            paint: prf_color,
            thickness: 1pt,
            dash: ("dot", 2pt)
        )
    ),
    inset: 4pt,
    breakable: true
)[
    #text(fill: prf_color, weight: "bold")[証明]
    #if title != none [
        text[ #title ]
    ]

    #body
]