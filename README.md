# DarcyLang :D
Fun Rust language implementation project.

## Updates
- This project is such a mess :|
- Tempted to rewrite the whole thing lol

## Project TODO
- [ ] Create abstract syntax tree from tokens (halfway done)
- [ ] Walk and evaluate the AST
- [ ] Finalize custom error handling and reporting
<<<<<<< HEAD
- [ ] Create infrastructure for handling context and scope
- [ ] Rewrite lexer to use `Arc<[T]>` instead of of `Vec<T>` 
 
=======
- [x] Create infrastructure for handling context and scope
- [x] Redo error reporting infrastructure (again)
- [ ] Eventually should redo the lexer (again)

>>>>>>> bd1a31444c128380d505fb63bcef4d74f948947a
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
<<<<<<< HEAD
- [x] Context-specific error reporting
- [x] Refactor error infrastructure (its very poorly organized)
- [x] Refactor error infrastructure to use enums with attached types

## Environments TODO
- [ ] Scope-dependent keywords
- [ ] Scope-dependent functions
=======
- [x] Error reporting should use enumerations and simpler reporting methods`
>>>>>>> bd1a31444c128380d505fb63bcef4d74f948947a

## Issues TODO
- [ ] Fix line numbers being out of sync between error and lexer

