# Rust-Tcp

## Setup
1. Open a terminal where the repository will be cloned

2. Install the Rust toolchain if not already installed
   - run the command
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
   - Hit enter again for the default installation
   - Check it was installed correctly by running the following commands
      ```bash
      rustc --version
      cargo --version
      ```

3. Clone the repository by running the following command
   ```git
      git clone https://github.com/CS-Personal-Data-Acquisition-Prototype/Rust-Tcp.git
   ```

4. Add the configuration file `config.toml` in `src` directory
   - Follow the [Configuration](#configuration) section for format guidelines

5. Run the program by following the [Usage](#usage) section

## Configuration
Add the configuration file `config.toml` in `src` directory, the config doesn't have any headers and the and default values look like this:
```toml
database_file = "data_acquisition.db"   # name of database file
local_addr = "0.0.0.0:7878"             # local address to listen for TCP requests on
```

## Usage
This crates defaults to a mock database connection when using `cargo build` or `cargo run`,<br>
to utilize an SQLite database the crate must be built and ran with `--features sql`.

1. Open a terminal in the directory 'tcp-server'.

2. To run the project in debug use the command:
`cargo run [--features sql]`


3. To run the project in release mode use the command:
`cargo run --release [--features sql]`

4. To see all the avaliable request and endpoint information, visit the [API Specification Document](https://docs.google.com/document/d/1tziVzWEAI0OJFBhgnmJrV8Y4_IoeSf7E4C9q4xEc57g/edit?usp=sharing).

# License Notice
To apply the Apache License to your work, attach the following boilerplate notice. The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

    Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
    
    http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
