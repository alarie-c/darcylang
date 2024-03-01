# DarcyLang
Fun Rust language implementation project.

## Updates
- Identifier errors work, yay!
- Lexer tokenizes the entire file before reporting errors

## Project TODO
- [ ] Create abstract syntax tree from tokens
- [ ] Walk and evaluate the AST
- [ ] Finalize custom error handling and reporting
- [ ] Create infrastructure for handling context and scope

## Tokenizer TODO
- [x] Cleanup code
- [ ] Reduce redundancy
- [x] Handle user errors (partially)
- [ ] Integrate context and scope
- [x] Get lines

## Errors TODO
- [x] Context-specific error reporting
- [x] Refactor error infrastructure (its very poorly organized)

## Issues TODO
- [ ] Fix line numbers being out of sync between error and lexer

