# md-to-tui
[![Rust](https://github.com/Horryportier/md-to-tui/actions/workflows/rust.yml/badge.svg)](https://github.com/Horryportier/md-to-tui/actions/workflows/rust.yml)

markdown to ratatui parser 

md-to-tui is an markdown parser from md like text to ratatui types 

#! its in the early stage and not evrything works  for exp. bold/itlic are not working right now 
becouse of crude and simple implemetion at the time. 

# Usage 
this library implements `MarkdownParsable` for types that implement `ToString` trait.
You can use `parse_markdown` fn to parse markdown to `Text`. 
`parse_markdown` takes `option` of `MdStyle` and returns `Result<Text<'static>, Error>`
```rust 
// example

let md = "
# TODO

- [ ] one
- [ ] two 

[link](http://exp.com)

"
let res = md.parse_markdown(Some(style))
```

# Road map 
- add support for bold and italic 
- make horizontal_rule 
- color number list 
- color headings according to its size 
- maybe add support for codeblock

