# Some architecture diagrams

## Execution architecture

```mermaid
sequenceDiagram
  command input ->> parser: text
  parser ->> component(radar weapon ...): cmd + id
  parser ->> result: id
  result -->> result: show spinner
component(radar weapon ...) ->> result: result + id
  result -->> result: show result
component(radar weapon ...) ->> next cmd: result + id
  parser ->> next cmd: cmd
  next cmd -->> component(radar weapon ...): start next execution
```

1. communications between components
2. component(radar weapon ...) holds tmp ref of objects so only can do execution
3. sperate state management from randering code

## Command / Code Runtime

```mermaid
C4Context
  Person(player ,"player")
  Boundary(zf, "ZF") {
    Boundary("godot", "godot") {
      Component(ui, "game UI", "ui")
      Component_Ext(engine, "godot engine", "rust")
      Component(vm, "vm", "vm")
      Component(wasmtime, "wasmtime", "wasm")
    }
  }

  BiRel(player, ui, "interact")
  Rel(ui, vm, "command / code")
  Rel(vm, wasmtime, "call")
  Rel(wasmtime, engine, "dispatch execution")
  Rel(engine, ui, "update ui")
  Rel(engine, vm, "update state")

  UpdateRelStyle(player, ui, $offsetX="10", $offsetY="5")
  UpdateRelStyle(vm, wasmtime, $offsetY="-20")
  UpdateRelStyle(engine, ui, $offsetX="-20", $offsetY="-20")
  UpdateRelStyle(engine, vm, $offsetX="-50", $offsetY="-20")
  UpdateRelStyle(wasmtime, engine, $offsetX="10")
  UpdateLayoutConfig($c4ShapeInRow="2")
```