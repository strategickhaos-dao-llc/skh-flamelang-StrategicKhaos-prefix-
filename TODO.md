# FlameLang v2.0.0 - TODO and Known Limitations

## Known Limitations

### Parser
- **Span Tracking**: Span end positions may be inaccurate in some expressions as they're calculated before token consumption. This doesn't affect parsing correctness but impacts error reporting precision.
- **Error Recovery**: Currently, the parser stops at the first error. Enhanced error recovery for multiple errors would improve developer experience.

### HIR (High-level IR)
- **Type Inference**: Currently defaults to `Type::Int` when no type annotation is provided. Proper type inference from initializer expressions is needed.
- **Type Checking**: Full type checking validation is not yet implemented.

### MIR (Mid-level IR)
- **Logical Operators**: `And` and `Or` operators are not fully implemented in MIR. They currently map to comparison operators as placeholders.
- **Control Flow**: Complex control flow (if/else, loops) lowering is partially implemented.

### CodeGen
- **LLVM Static Linking**: Requires LLVM with static libraries including libPolly. This limits some deployment scenarios.
- **Target Triple**: Currently hardcoded; should support multiple target architectures.
- **Optimization**: LLVM optimization passes not yet integrated.

### CLI
- **JIT Execution**: `flamecc run` is not fully implemented (no execution engine yet).
- **Formatter**: `flamecc fmt` is a placeholder (validates syntax but doesn't format).

## Planned Features

### Short-term (v2.1.0)
- [ ] Complete type inference in HIR
- [ ] Full control flow support in MIR (if/else, while, for)
- [ ] Better error messages with source location
- [ ] String literal support in codegen
- [ ] Basic optimization passes

### Medium-term (v2.2.0)
- [ ] Module system implementation
- [ ] Trait system support
- [ ] Generic functions and types
- [ ] Memory safety analysis
- [ ] REPL (Read-Eval-Print Loop)

### Long-term (v3.0.0)
- [ ] Incremental compilation
- [ ] Language server protocol (LSP) support
- [ ] Package manager integration
- [ ] Advanced optimization (inlining, constant folding)
- [ ] WebAssembly target support

## Architecture Improvements

### Parser
- [ ] Switch to lalrpop for better maintainability
- [ ] Add precedence climbing for expressions
- [ ] Improve span tracking accuracy

### HIR
- [ ] Implement Hindley-Milner type inference
- [ ] Add lifetime analysis
- [ ] Borrowck-like ownership checking

### MIR
- [ ] SSA transformation
- [ ] Dead code elimination
- [ ] Constant propagation
- [ ] Loop optimizations

### CodeGen
- [ ] Support for multiple LLVM versions
- [ ] Dynamic linking option
- [ ] Cross-compilation support
- [ ] Debug information generation

## Performance

- [ ] Benchmark lexer performance
- [ ] Profile compilation pipeline
- [ ] Optimize AST representation
- [ ] Reduce memory allocations

## Documentation

- [ ] Language specification document
- [ ] API documentation for all crates
- [ ] Tutorial series
- [ ] Best practices guide

## Testing

- [ ] Integration tests for full pipeline
- [ ] Fuzzing for parser robustness
- [ ] Benchmark suite
- [ ] Example programs library

## Community

- [ ] Contribution guidelines
- [ ] Code of conduct
- [ ] Issue templates
- [ ] Discord/Slack community

---

*This is a living document. Items will be added/removed as the project evolves.*
