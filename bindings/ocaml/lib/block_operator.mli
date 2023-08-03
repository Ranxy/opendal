val new_operator :
  Scheme.scheme ->
  (string * string) list ->
  (Inner.blocking_operator, string) result
(** [new_operator schema config_map] Create a new block operator from given scheme and config_map.
    
    @param schema The support service
    @param config_map Configuration information required by the target service
    @return The block operator
*)

val is_exist : Inner.blocking_operator -> string -> (bool, string) result
(** [is_exist operator path] Check if this path exists or not.
    
    @param operator The block operator
    @param path want to check
    @return is exists
*)

val create_dir : Inner.blocking_operator -> string -> (bool, string) result
(** [create_dir operator path] Create a dir at given path.
    
    # Notes
    
    To indicate that a path is a directory, it is compulsory to include
    a trailing / in the path. Failure to do so may result in
    `NotADirectory` error being returned by OpenDAL.
    
    # Behavior
    
    - Create on existing dir will succeed.
    - Create dir is always recursive, works like `mkdir -p`
    @param operator The block operator
    @param path want to create dir
*)

val read : Inner.blocking_operator -> string -> (char array, string) result
(** [read operator path] Read the whole path into a bytes.
    
    @param operator The block operator
    @param path want to read
    @return data of path
*)

val write : Inner.blocking_operator -> string -> bytes -> (unit, string) result
(** [write operator path data] Write bytes into given path.
    - Write will make sure all bytes has been written, or an error will be returned.
    @param operator The block operator
    @param path want to write
    @param data want to write
*)

val copy : Inner.blocking_operator -> string -> string -> (unit, string) result
(** [copy operator from to] Copy a file from [from] to [to].
    - [from] and [to] must be a file.
    - [to] will be overwritten if it exists.
    - If [from] and [to] are the same, nothing will happen.
    - copy is idempotent. For same [from] and [to] input, the result will be the same.
    @param operator The block operator
    @param from file path
    @param to file path
*)

val rename :
  Inner.blocking_operator -> string -> string -> (unit, string) result
(** [rename operator from to] Rename a file from [from] to [to].
    - [from] and [to] must be a file.
    - [to] will be overwritten if it exists.
    - If [from] and [to] are the same, a `IsSameFile` error will occur.
    @param operator The block operator
    @param from file path
    @param to file path
*)

val delete : Inner.blocking_operator -> string -> (unit, string) result
(** [delete operator path] Delete given path.
    - Delete not existing error won't return errors.
    @param operator The block operator
    @param path file path
*)

val remove : Inner.blocking_operator -> string array -> (unit, string) result
(** [remove operator paths] Remove path array.
    - We don't support batch delete now, will call delete on each object in turn
    @param operator The block operator
    @param paths file path array
*)

val remove_all : Inner.blocking_operator -> string -> (unit, string) result
(** [remove_all operator path] Remove the path and all nested dirs and files recursively.
    - We don't support batch delete now, will call delete on each object in turn
    @param operator The block operator
    @param path file path
*)
