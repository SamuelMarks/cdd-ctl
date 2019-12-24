# CDD JSON-RPC APIs

## Getting started

- This api accepts `POST` requests exposed via JSON-RPC 2.0. You may wish to read the [spec](http://www.jsonrpc.org/specification).

Example request:
```json
{"jsonrpc" : "2.0",  "method" : "listModels",  "params" : [ ]}
```

Example response:
```json
{"jsonrpc" : "2.0", "result" : [{ "name" : "Pet", "age" : 45 }]
```

## Authentication
Currently the api does not require authentication, as it is primarily used in a controlled environment. This may change in future as remote requests become necessary.

## Methods

### listModels
Lists all models in a project.

#### Parameters
/none/
