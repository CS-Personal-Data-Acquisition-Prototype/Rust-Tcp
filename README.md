# Rust-Tcp
Navigate into the tcp-server file in a terminal

To download packages & dependencies use: cargo build

To run the project use: cargo run
To run in release mode: cargo run --release

## Configuration
Add the configuration file `config.toml` in `src` directory, the config doesn't have any headers and the and default values look like this:
```toml
database_path = ""          # path to database file
local_addr = "0.0.0.0:7878" # local address to listen for TCP requests on
```

# License Notice
To apply the Apache License to your work, attach the following boilerplate notice. The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

    Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
    
    http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
