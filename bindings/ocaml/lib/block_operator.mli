(** [schema] is the backend service name which we want to connection. *)
val new_operator :
  string ->
 (string * string) list ->  (Inner.blocking_operator, string) result
val is_exist : Inner.blocking_operator -> string -> (bool, string) result
val create_dir : Inner.blocking_operator -> string -> (bool, string) result
val read : Inner.blocking_operator -> string -> (char array, string) result
val write :
  Inner.blocking_operator -> string -> bytes -> (unit, string) result
val copy :
  Inner.blocking_operator -> string -> string -> (unit, string) result
val rename :
  Inner.blocking_operator -> string -> string -> (unit, string) result
val delete : Inner.blocking_operator -> string -> (unit, string) result
val remove : Inner.blocking_operator -> string array -> (unit, string) result
val remove_all : Inner.blocking_operator -> string -> (unit, string) result
