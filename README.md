# daas: demangler-as-a-service

demangler-as-a-service for demangling C++ symbols quickly. Usage is dead simple and impossible to fuck up. Or maybe not.

```shell
$ curl -s http://localhost:8000/
Make a GET call with /<mangled_symbol> to return the demangled form
$ curl -s http://localhost:8000/_ZN6icu_6011StringPieceC1EPKc
icu_60::StringPiece::StringPiece(char const*)
$ curl -s http://localhost:8000/_ZN6icu_6011StringPieceC1EPKc/json | jq .
{
  "symbol": "_ZN6icu_6011StringPieceC1EPKc",
  "result": "icu_60::StringPiece::StringPiece(char const*)"
}
```
