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