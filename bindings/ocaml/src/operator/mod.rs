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

mod _type;
mod metadata;
mod reader;

use _type::*;

use super::*;

fn add_layers(mut op: od::Operator, layers: Vec<ocaml::Pointer< layers::Layer>>) -> od::Operator {
    for layer in layers {
        let l: layers::Layer;
        unsafe {
            l = std::ptr::read(layer.as_ptr()); //bug, will panic when gc
        }
        match l {
            layers::Layer::Retry(layers::RetryLayer(inner)) => op = op.layer(inner),
            layers::Layer::ImmutableIndex(layers::ImmutableIndexLayer(inner)) => {
                op = op.layer(inner)
            }
            layers::Layer::ConcurrentLimit(layers::ConcurrentLimitLayer(inner)) => {
                op = op.layer(inner)
            }
            layers::Layer::Timeout(layers::TimeoutLayer(inner)) => op = op.layer(inner),
            layers::Layer::Logging(layers::LoggingLayer(inner)) => op = op.layer(inner),
        }
    }
    op
}

#[ocaml::func]
#[ocaml::sig(
    "string -> (string * string) list -> Layers.layer array -> (operator, string) Result.t "
)]
pub fn operator(
    scheme_str: String,
    map: BTreeMap<String, String>,
    layers: Vec<ocaml::Pointer<layers::Layer>>,
) -> Result<ocaml::Pointer<Operator>, String> {
    let op = map_res_error(new_operator(scheme_str, map))?;

    let op = add_layers(op,layers );
    Ok(Operator(op.blocking()).into())
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (metadata, string) Result.t ")]
pub fn blocking_stat(
    operator: &mut Operator,
    path: String,
) -> Result<ocaml::Pointer<Metadata>, String> {
    map_res_error(operator.0.stat(path.as_str()).map(|m| Metadata(m).into()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (bool, string) Result.t ")]
pub fn blocking_is_exist(operator: &mut Operator, path: String) -> Result<bool, String> {
    map_res_error(operator.0.is_exist(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (bool, string) Result.t ")]
pub fn blocking_create_dir(operator: &mut Operator, path: String) -> Result<(), String> {
    map_res_error(operator.0.create_dir(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (char array, string) Result.t ")]
pub fn blocking_read(operator: &mut Operator, path: String) -> Result<Vec<u8>, String> {
    map_res_error(operator.0.read(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (reader, string) Result.t ")]
pub fn blocking_reader(
    operator: &mut Operator,
    path: String,
) -> Result<ocaml::Pointer<Reader>, String> {
    map_res_error(operator.0.reader(path.as_str())).map(|op| Reader(op).into())
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> bytes -> (unit, string) Result.t ")]
pub fn blocking_write(
    operator: &mut Operator,
    path: String,
    bs: &'static [u8],
) -> Result<(), String> {
    map_res_error(operator.0.write(path.as_str(), bs))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> string -> (unit, string) Result.t ")]
pub fn blocking_copy(operator: &mut Operator, from: String, to: String) -> Result<(), String> {
    map_res_error(operator.0.copy(from.as_str(), to.as_str()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> string -> (unit, string) Result.t ")]
pub fn blocking_rename(operator: &mut Operator, from: String, to: String) -> Result<(), String> {
    map_res_error(operator.0.rename(from.as_str(), to.as_str()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (unit, string) Result.t ")]
pub fn blocking_delete(operator: &mut Operator, path: String) -> Result<(), String> {
    map_res_error(operator.0.delete(path.as_str()))
}

#[ocaml::func]
#[ocaml::sig("operator -> string array -> (unit, string) Result.t ")]
pub fn blocking_remove(operator: &mut Operator, path: Vec<String>) -> Result<(), String> {
    map_res_error(operator.0.remove(path))
}

#[ocaml::func]
#[ocaml::sig("operator -> string -> (unit, string) Result.t ")]
pub fn blocking_remove_all(operator: &mut Operator, path: String) -> Result<(), String> {
    map_res_error(operator.0.remove_all(path.as_str()))
}
