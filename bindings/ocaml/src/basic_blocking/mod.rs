// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::*;

#[ocaml::sig]
pub struct BlockingOperator(od::BlockingOperator);
ocaml::custom!(BlockingOperator);

#[ocaml::func]
#[ocaml::sig("Scheme.scheme -> (string * string) list -> (blocking_operator, string) Result.t ")]
pub fn constructor(
    scheme: scheme::Scheme,
    map: BTreeMap<String, String>,
) -> Result<ocaml::Pointer<BlockingOperator>, String> {
    let op = map_res_error(new_operator(scheme, map))?;
    Ok(BlockingOperator(op.blocking()).into())
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> (bool, string) Result.t ")]
pub fn is_exist(operator: &mut BlockingOperator, path: String) -> Result<bool, String> {
    map_res_error(operator.0.is_exist(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> (bool, string) Result.t ")]
pub fn create_dir(operator: &mut BlockingOperator, path: String) -> Result<(), String> {
    map_res_error(operator.0.create_dir(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> (char array, string) Result.t ")]
pub fn read(operator: &mut BlockingOperator, path: String) -> Result<Vec<u8>, String> {
    map_res_error(operator.0.read(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> bytes -> (unit, string) Result.t ")]
pub fn write(
    operator: &mut BlockingOperator,
    path: String,
    bs: &'static [u8],
) -> Result<(), String> {
    map_res_error(operator.0.write(path.as_str(), bs))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> string -> (unit, string) Result.t ")]
pub fn copy(
    operator: &mut BlockingOperator,
    from: String,
    to: String,
) -> Result<(), String> {
    map_res_error(operator.0.copy(from.as_str(), to.as_str()))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> string -> (unit, string) Result.t ")]
pub fn rename(
    operator: &mut BlockingOperator,
    from: String,
    to: String,
) -> Result<(), String> {
    map_res_error(operator.0.rename(from.as_str(), to.as_str()))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> (unit, string) Result.t ")]
pub fn delete(operator: &mut BlockingOperator, path: String) -> Result<(), String> {
    map_res_error(operator.0.delete(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string array -> (unit, string) Result.t ")]
pub fn remove(operator: &mut BlockingOperator, path: Vec<String>) -> Result<(), String> {
    map_res_error(operator.0.remove(path))
}

#[ocaml::func]
#[ocaml::sig("blocking_operator -> string -> (unit, string) Result.t ")]
pub fn remove_all(operator: &mut BlockingOperator, path: String) -> Result<(), String> {
    map_res_error(operator.0.remove_all(path.as_str()))
}
