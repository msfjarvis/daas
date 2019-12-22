# daas: demangler-as-a-service

demangler-as-a-service for demangling C++ symbols quickly. Usage is dead simple and impossible to fuck up. Or maybe not.

A public instance is available at https://daas.msfjarvis.dev.

```shell
$ curl -s https://daas.msfjarvis.dev/
Make a GET call with /<mangled_symbol> to return the demangled form
$ curl -s https://daas.msfjarvis.dev/_ZN6icu_6011StringPieceC1EPKc
icu_60::StringPiece::StringPiece(char const*)
```
