let new_operator (schema : Scheme.scheme) config_map =
  Inner.new_blocking_operator (Scheme.scheme_str schema) config_map

let is_exist = Inner.blocking_is_exist
let create_dir = Inner.blocking_create_dir
let read = Inner.blocking_read
let write = Inner.blocking_write
let copy = Inner.blocking_copy
let rename = Inner.blocking_rename
let delete = Inner.blocking_delete
let remove = Inner.blocking_remove
let remove_all = Inner.blocking_remove_all
