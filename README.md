# DarcyLang
Fun Rust language implementation project.

## Updates
- This project is such a mess :|
- Tempted to rewrite the whole thing lol

## Project TODO
- [ ] Create abstract syntax tree from tokens (halfway done)
- [ ] Walk and evaluate the AST
- [ ] Finalize custom error handling and reporting
- [ ] Create infrastructure for handling context and scope
- [ ] Rewrite lexer to use `Arc<[T]>` instead of of `Vec<T>` 
 
## Tokenizer TODO
- [x] Cleanup code
- [ ] Reduce redundancy
- [x] Handle user errors (partially)
- [ ] Integrate context and scope
- [x] Get lines

## Errors TODO
- [x] Context-specific error reporting
- [x] Refactor error infrastructure (its very poorly organized)
- [x] Refactor error infrastructure to use enums with attached types

## Environments TODO
- [ ] Scope-dependent keywords
- [ ] Scope-dependent functions

## Issues TODO
- [ ] Fix line numbers being out of sync between error and lexer

