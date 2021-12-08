/*
 * $Id$
 *
 * Copyright 2021 Purushottam A. Kulkarni. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 * this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation and
 * or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors
 * may be used to endorse or promote products derived from this software without
 * specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE
 *
 */

use std::{env, fs, path::PathBuf, process::Command};

const LIB: &str = "jitterentropy";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_path = out_path.join("jitterentropy-library");
    let wrapper_path = build_path.join("wrapper.h");

    Command::new("/bin/cp")
        .args(&["-r", "jitterentropy-library", build_path.to_str().unwrap()])
        .status()
        .expect("Failed to copy dir");

    make_cmd::make()
        .args(&["-C", build_path.to_str().unwrap()])
        .status()
        .expect("Failed to make");

    fs::copy("wrapper.h", wrapper_path.clone()).expect("Failed to copy wrapper.h");

    println!("cargo:rustc-link-search=native={}", build_path.display());
    println!("cargo:rustc-link-lib={}", LIB);
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapper_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for lib{}", LIB));

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write libjitterentropy bindings");
}
