# Commands

## Design rules: 

- no quoating
  - allows input string -> vec of args

## mission

---

### `mission summary`

#### definition

show mission info/details

#### returns

text / renderable rich media?

---

### `mission targets`

#### definition

show mission info/details

#### returns

`Vec<Vector3>`

## Flight control

---

### `engine start`

#### definition

start engine

#### rules

- engine must be started to provide thruster power.
- when engine is on, current unit will be show on radar
- if started returns `AlreadyStartedErr`

#### returns

`Result<(), AlreadyStartedErr>`

---

### `engine stop`

#### definition

stop engine

#### rules

- when engine is stopped, current unit will not be show on radar
- if started returns `NotStartedErr`

#### returns

`Result<(), NotStartedErr>`

---

### `engine thruster <percentage>`

#### definition

set current engine thruster power

#### example

`engine thruster 20%`

#### returns

`Result<(), NotStartedErr>`

---

### `steering rotate -x <deg> -y <deg> -x <deg> --keep`

#### definition

set current steering

#### example

`steering rotate -x 20`

#### returns

[]

---


#### definition
#### returns

[]

---

### `steering mode manual`

#### definition
#### returns

[]

---

### `autopilot -t,--target <target>`

#### definition
#### returns

[]

---

### `autopilot -t base`

#### definition
#### returns

[]

---

### `locate`

#### definition
#### returns

[]

