type location = {
  start : int;
  end_ : int;
  filename : string;
}

type error = {
  message: string;
  full_text: string;
  location: location
}

type var = {
  text: string;
  location: string;
}

type binary_op =
  | Add
  | Sub
  | Mul
  | Div
  | Rem
  | Eq
  | Neq
  | Lt
  | Gt
  | Lte
  | Gte
  | And
  | Or

type term = 
  | Error of { message: string; full_text: string; location: location }
  | Int of { value: int; location: location }
  | Str of { value: string; location: location }
  | Call of { callee: term; arguments: term list; location: location }
  | Binary of { lhs: term; op: binary_op; rhs: term; location: location }
  | Function of { parameters: term list; value: term; location: location }
  | Let of { name: var; value: term; next: term; location: location }
  | If of { condition: term; then_: term; otherwise: term; location: location }
  | Print of { value: term; location: location }
  | First of { value: term; location: location }
  | Second of {value: term; location: location }
  | Bool of { value: bool; location: location }
  | Tuple of { first: term; second: term; location: location }
  | Var of var

type file = {
  name: string;
  expression: term;
  location: location;
}
