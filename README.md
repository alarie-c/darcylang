# DarcyLang :D
Fun Rust language implementation project.

## Updates
- Identifier errors work, yay!
- Lexer tokenizes the entire file before reporting errors

## Project TODO
- [ ] Create abstract syntax tree from tokens
- [ ] Walk and evaluate the AST
- [ ] Finalize custom error handling and reporting
- [x] Create infrastructure for handling context and scope
- [ ] Redo error reporting infrastructure (again)

## Tokenizer TODO
- [x] Cleanup code
- [ ] Reduce redundancy
- [x] Handle user errors (partially)
- [ ] Integrate context and scope
- [x] Get lines

## Scope TODO
- [x] Environment and context infrastructure
- [ ] Find values in memory from environment and parent environments 

## Errors TODO
- [ ] Error reporting should use enumerations and simpler reporting methods`

## Issues TODO
- [ ] Fix line numbers being out of sync between error and lexer

