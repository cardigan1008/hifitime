/*
 * Hifitime, part of the Nyx Space tools
 * Copyright (C) 2022 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Apache
 * v. 2.0. If a copy of the Apache License was not distributed with this
 * file, You can obtain one at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * Documentation: https://nyxspace.com/
 */

use core::str::FromStr;

use pyo3::prelude::*;

use crate::prelude::*;

#[pymethods]
impl Epoch {
    #[new]
    fn new(string_repr: String) -> PyResult<Self> {
        match Epoch::from_str(&string_repr) {
            Ok(e) => Ok(e),
            Err(e) => Err(PyErr::from(e)),
        }
    }
}
