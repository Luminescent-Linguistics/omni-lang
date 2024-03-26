//!     Omni-lang builtins, the builtin libraries for the Omni programming language.
//!     Copyright (C) 2024  Frank Hudson
//!
//!     This program is free software: you can redistribute it and/or modify
//!     it under the terms of the GNU General Public License as published by
//!     the Free Software Foundation, either version 3 of the License, or
//!     (at your option) any later version.
//!
//!     This program is distributed in the hope that it will be useful,
//!     but WITHOUT ANY WARRANTY; without even the implied warranty of
//!     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!     GNU General Public License for more details.
//!
//!     You should have received a copy of the GNU General Public License
//!     along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//!
//!     You can contact the author at <frank.hudson.v0@gmail.com>.

use color_eyre::eyre::{Result, eyre};
use std::{
    env,
    fs::File,
    io::prelude::*,
    path::Path
};

fn main() -> Result<()> {
    // Install `color_eyre`.
    color_eyre::install()?;
    
    // Collect commandline arguments.
    let args: Vec<String> = env::args().collect();

    // Ensure there are exactly 2 arguments.
    // (To be changed when options are added)
    if args.len() != 2 { return Err(eyre!("Invalid number of arguments; Expected 1\nExpected Format: omni <file path>")); }

    // Separate the file path.
    let target_path = Path::new(&args[1]);
    // Attempt to open the file at the path in read-only mode.
    let mut target_file = match File::open(target_path) {
        Err(why) => return Err(eyre!("Couldn't open the file at the path \"{}\"; \n{}", &args[1], why)),
        Ok(file) => file,
    };
    // Read the contents into a string.
    let mut contents = String::new();
    if let Err(why) = target_file.read_to_string(&mut contents) {
        return Err(eyre!("Error reading the file {}; \n{}", &args[1], why))
    }

    println!("{contents}");

    Ok(())
}
