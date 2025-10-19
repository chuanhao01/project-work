#import "@preview/pintorita:0.1.4"

// #set page(height: auto, width: auto, fill: black, margin: 2em)
// #set text(fill: white)

#show raw.where(lang: "pintora"): it => pintorita.render(it.text)

= pintora

is this now a normal page
