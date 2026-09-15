use crate::pipeline::cells::cell::CellSignature;
use crate::pipeline::cells::cell_cmd::CellCommand;
use crate::pipeline::cells::cell_dependency::CellDependency;
use crate::pipeline::cells::cell_registry::CellRegistry;
use crate::pipeline::driver::registry_parser::error::{RegistryParseError, RegistryParseErrors};
use crate::utility::common::vec_add::AddableVec;
use crate::utility::vpe::VPE;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Add;
use std::path::{Path, PathBuf};

pub struct CellRegisterParsed<V> {
    pub value: V,
    pub position: CellSignature,
    pub error: RegistryParseErrors,
}

impl<V> VPE<V> for CellRegisterParsed<V> {
    type P = CellSignature;
    type E = RegistryParseErrors;

    fn get_value(&self) -> &V {
        &self.value
    }
    fn get_position(&self) -> &Self::P {
        &self.position
    }
    fn get_errors(&self) -> &Self::E {
        &self.error
    }

    fn transfer_vpe_ownership(self) -> (V, Self::P, Self::E) {
        (self.value, self.position, self.error)
    }

    fn set_value(&mut self, value: V) {
        self.value = value
    }
    fn set_position(&mut self, position: Self::P) {
        self.position = position
    }
    fn set_error(&mut self, error: Self::E) {
        self.error = error
    }

    fn new(value: V, position: Self::P, error: Self::E) -> Self {
        Self {
            value,
            position,
            error,
        }
    }
}

impl Add for CellSignature {
    type Output = CellSignature;

    fn add(self, rhs: Self) -> Self::Output {
        self
    }
}

impl<V> CellRegisterParsed<V> {
    pub fn new_value(value: V, position: CellSignature) -> Self {
        Self {
            value,
            position,
            error: AddableVec::from(vec![]),
        }
    }

    pub fn new_error(position: CellSignature, error: RegistryParseError) -> CellRegisterParsed<()> {
        CellRegisterParsed {
            value: (),
            position,
            error: AddableVec::from(vec![error]),
        }
    }

    pub fn new_error_with_default(
        default: V,
        position: CellSignature,
        error: RegistryParseError,
    ) -> Self {
        Self {
            value: default,
            position,
            error: AddableVec::from(vec![error]),
        }
    }

    pub fn push_vec(
        vec_parsed: Vec<CellRegisterParsed<V>>,
        vec_position: CellSignature,
    ) -> CellRegisterParsed<Vec<V>> {
        let mut p_list: CellRegisterParsed<Vec<V>> =
            CellRegisterParsed::new_value(vec![], vec_position);
        for parsed in vec_parsed {
            p_list = CellRegisterParsed::<Vec<V>>::merge_same_pe(p_list, parsed, |list, parsed| {
                list.into_iter().chain(vec![parsed].into_iter()).collect()
            });
        }
        p_list
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonDependency {
    name: String,
    version: String,
    source: Option<String>,
    alias: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonCommand {
    command: String,
    operation: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonCellRegistry {
    name: String,
    version: String,
    author: Option<Vec<String>>,
    description: Option<String>,
    dependencies: Option<Vec<JsonDependency>>,
    commands: Option<Vec<JsonCommand>>,
}

pub struct RegistryParser {
    context: String,
    position: CellSignature,
    path: PathBuf,
}

impl RegistryParser {
    pub fn new_value<V>(&self, value: V) -> CellRegisterParsed<V> {
        CellRegisterParsed::new_value(value, self.position.clone())
    }

    pub fn new_error<F>(&self, f_err: F) -> CellRegisterParsed<()>
    where
        F: FnOnce(PathBuf, CellSignature) -> RegistryParseError,
    {
        CellRegisterParsed::<()>::new_error(
            self.position.clone(),
            f_err(self.path.clone(), self.position.clone()),
        )
    }

    pub fn new_error_with_default<V, F>(&self, default: V, f_err: F) -> CellRegisterParsed<V>
    where
        F: FnOnce(PathBuf, CellSignature) -> RegistryParseError,
    {
        CellRegisterParsed::new_error_with_default(
            default,
            self.position.clone(),
            f_err(self.path.clone(), self.position.clone()),
        )
    }
}

impl RegistryParser {
    pub fn new(position: Option<CellSignature>, path: PathBuf) -> Self {
        Self {
            context: Default::default(),
            position: position.unwrap_or(CellSignature {
                name: "".to_string(),
                version: (0, 0, 0),
            }),
            path,
        }
    }

    pub fn parse(
        position: Option<CellSignature>,
        path: PathBuf,
    ) -> CellRegisterParsed<Option<CellRegistry>> {
        RegistryParser::new(position, path).self_parse()
    }

    fn self_parse(&mut self) -> CellRegisterParsed<Option<CellRegistry>> {
        let context_res = fs::read_to_string(&self.path);

        match context_res {
            Ok(context) => self.context = context,
            Err(_) => {
                return self.new_error_with_default(None, RegistryParseError::file_read_failure)
            }
        };

        let json_cell: JsonCellRegistry = match serde_json::from_str(self.context.as_str()) {
            Ok(v) => v,
            Err(_) => return self.new_error_with_default(None, RegistryParseError::parse_failure),
        };

        if json_cell.name == "" {
            return self.new_error_with_default(None, RegistryParseError::empty_name);
        }

        let mut signature: CellRegisterParsed<CellSignature> =
            CellRegisterParsed::<CellSignature>::lift_same_pe(
                self.parse_version(&json_cell.version),
                |v| CellSignature {
                    name: json_cell.name,
                    version: v,
                },
            );

        if self.position.name == "".to_string() {
            self.position = signature.value.clone();
        } else if signature.value != self.position.clone() {
            signature = CellRegisterParsed::<CellSignature>::merge_same_pe(
                signature,
                self.new_error(RegistryParseError::signature_mismatch),
                |sign, _| sign,
            )
        }

        let dependencies: CellRegisterParsed<Vec<CellDependency>> = CellRegisterParsed::push_vec(
            json_cell
                .dependencies
                .unwrap_or(vec![])
                .into_iter()
                .map(|dep| {
                    CellRegisterParsed::<CellDependency>::lift_same_pe(
                        self.parse_version(&dep.version),
                        |v| CellDependency {
                            source: dep.source.map(|v| v.parse().unwrap()),
                            signature: CellSignature {
                                name: dep.name,
                                version: v,
                            },
                            alias: dep.alias,
                        },
                    )
                })
                .collect(),
            signature.value.clone(),
        );

        let commands: CellRegisterParsed<Vec<CellCommand>> = CellRegisterParsed::push_vec(
            json_cell
                .commands
                .unwrap_or(vec![])
                .into_iter()
                .map(|cmd| {
                    CellRegisterParsed::<CellCommand>::lift_same_pe(
                        self.parse_operation(&cmd.operation),
                        |operation| CellCommand {
                            command: cmd.command,
                            operation,
                        },
                    )
                })
                .collect(),
            signature.value.clone(),
        );

        CellRegisterParsed::<Option<CellRegistry>>::merge_3_same_pe(
            signature,
            dependencies,
            commands,
            |signature, dependencies, commands| {
                Some(CellRegistry {
                    signature: signature,
                    authors: json_cell.author.unwrap_or(vec![]),
                    description: json_cell.description.unwrap_or("".parse().unwrap()),
                    dependencies,
                    commands,
                })
            },
        )
    }
}
