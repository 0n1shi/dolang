" Vim syntax file
" Language: Dolang
" Maintainer: Kazuki ONISHI

if exists("b:current_syntax")
  finish
endif

" Keywords
syntax keyword dolangKeyword let fn if else for in match return and or not
syntax keyword dolangBuiltIn print println map filter append
syntax keyword dolangBoolean true false

" Operators
syntax match dolangOperator /->/
syntax match dolangOperator /==/
syntax match dolangOperator /!=/
syntax match dolangOperator /<=/
syntax match dolangOperator />=/
syntax match dolangOperator /[=+\-*\/%<>|]/
syntax match dolangPipeOperator /|>/

" Identifiers
syntax match dolangIdentifier /\<[a-zA-Z_][a-zA-Z0-9_]*\>/

" Numbers
syntax match dolangNumber /\v-?\d+(\.\d+)?/

" Strings
syntax region dolangString start=/"/ end=/"/ contains=dolangEscape
syntax match dolangEscape /\\./

" Comments (if you plan to support them)
" syntax match dolangComment /#.*$/

" Brackets
syntax match dolangBracket /[\[\]{}()]/
syntax match dolangComma /,/
syntax match dolangDot /\./

" Highlighting links
highlight link dolangKeyword Keyword
highlight link dolangBuiltIn Keyword
highlight link dolangBoolean Boolean
highlight link dolangOperator Operator
highlight link dolangPipeOperator Special
highlight link dolangIdentifier Identifier
highlight link dolangNumber Number
highlight link dolangString String
highlight link dolangEscape SpecialChar
highlight link dolangBracket Delimiter
highlight link dolangComma Delimiter
highlight link dolangDot Delimiter
" highlight link dolangComment Comment

let b:current_syntax = "dolang"
