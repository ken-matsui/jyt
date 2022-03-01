use crate::ext;

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;

/// # Safety
///
/// This public function might dereference a raw pointer.
#[no_mangle]
pub unsafe extern "C" fn to_json(from: ext::Ext, text: *const c_char) -> *const c_char {
    let text_c_str = CStr::from_ptr(text);
    let value = ext::deserialize::<ext::json::Value>(from, text_c_str.to_str().unwrap());
    let output = CString::new(ext::json::serialize(&value.unwrap()).unwrap()).unwrap();
    let ptr = output.as_ptr();
    mem::forget(output);
    ptr
}

/// # Safety
///
/// This public function might dereference a raw pointer.
#[no_mangle]
pub unsafe extern "C" fn to_yaml(from: ext::Ext, text: *const c_char) -> *const c_char {
    let text_c_str = CStr::from_ptr(text);
    let value = ext::deserialize::<ext::yaml::Value>(from, text_c_str.to_str().unwrap());
    let output = CString::new(ext::yaml::serialize(&value.unwrap()).unwrap()).unwrap();
    let ptr = output.as_ptr();
    mem::forget(output);
    ptr
}

/// # Safety
///
/// This public function might dereference a raw pointer.
#[no_mangle]
pub unsafe extern "C" fn to_toml(from: ext::Ext, text: *const c_char) -> *const c_char {
    let text_c_str = CStr::from_ptr(text);
    let value = ext::deserialize::<ext::toml::Value>(from, text_c_str.to_str().unwrap());
    let output = CString::new(ext::toml::serialize(&value.unwrap()).unwrap()).unwrap();
    let ptr = output.as_ptr();
    mem::forget(output);
    ptr
}

#[cfg(test)]
pub(crate) mod tests {
    use inline_c::assert_c;

    pub(crate) static JSON: &str = r#"{
  "title": "TOML Example",
  "owner": {
    "name": "Tom Preston-Werner"
  },
  "database": {
    "server": "192.168.1.1",
    "ports": [
      8000,
      8001,
      8002
    ],
    "connection_max": 5000,
    "enabled": true
  }
}"#;

    pub(crate) static YAML: &str = r#"---
title: TOML Example
owner:
  name: Tom Preston-Werner
database:
  server: 192.168.1.1
  ports:
    - 8000
    - 8001
    - 8002
  connection_max: 5000
  enabled: true
"#;

    pub(crate) static TOML: &str = r#"title = "TOML Example"

[owner]
name = "Tom Preston-Werner"

[database]
server = "192.168.1.1"
ports = [8000, 8001, 8002]
connection_max = 5000
enabled = true
"#;

    #[test]
    fn c_api_convert_json_to_yaml() {
        (assert_c! {
            #include "jyt.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input =
                    "{\n"
                    "  \"title\": \"TOML Example\",\n"
                    "  \"owner\": {\n"
                    "    \"name\": \"Tom Preston-Werner\"\n"
                    "  },\n"
                    "  \"database\": {\n"
                    "    \"server\": \"192.168.1.1\",\n"
                    "    \"ports\": [\n"
                    "      8000,\n"
                    "      8001,\n"
                    "      8002\n"
                    "    ],\n"
                    "    \"connection_max\": 5000,\n"
                    "    \"enabled\": true\n"
                    "  }\n"
                    "}";
                const char* output = to_yaml(Json, input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout(YAML.to_string());
    }
    #[test]
    fn c_api_convert_json_to_toml() {
        (assert_c! {
            #include "jyt.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input =
                    "{\n"
                    "  \"title\": \"TOML Example\",\n"
                    "  \"owner\": {\n"
                    "    \"name\": \"Tom Preston-Werner\"\n"
                    "  },\n"
                    "  \"database\": {\n"
                    "    \"server\": \"192.168.1.1\",\n"
                    "    \"ports\": [\n"
                    "      8000,\n"
                    "      8001,\n"
                    "      8002\n"
                    "    ],\n"
                    "    \"connection_max\": 5000,\n"
                    "    \"enabled\": true\n"
                    "  }\n"
                    "}";
                const char* output = to_toml(Json, input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout(TOML.to_string());
    }

    #[test]
    fn c_api_convert_yaml_to_json() {
        (assert_c! {
            #include "jyt.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input =
                    "---\n"
                    "title: TOML Example\n"
                    "owner:\n"
                    "  name: Tom Preston-Werner\n"
                    "database:\n"
                    "  server: 192.168.1.1\n"
                    "  ports:\n"
                    "    - 8000\n"
                    "    - 8001\n"
                    "    - 8002\n"
                    "  connection_max: 5000\n"
                    "  enabled: true\n";
                const char* output = to_json(Yaml, input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout(JSON.to_string());
    }
    #[test]
    fn c_api_convert_yaml_to_toml() {
        (assert_c! {
            #include "jyt.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input =
                    "---\n"
                    "title: TOML Example\n"
                    "owner:\n"
                    "  name: Tom Preston-Werner\n"
                    "database:\n"
                    "  server: 192.168.1.1\n"
                    "  ports:\n"
                    "    - 8000\n"
                    "    - 8001\n"
                    "    - 8002\n"
                    "  connection_max: 5000\n"
                    "  enabled: true\n";
                const char* output = to_toml(Yaml, input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout(TOML.to_string());
    }

    #[test]
    fn c_api_convert_toml_to_json() {
        (assert_c! {
            #include "jyt.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input =
                    "title = \"TOML Example\"\n"
                    "\n"
                    "[owner]\n"
                    "name = \"Tom Preston-Werner\"\n"
                    "\n"
                    "[database]\n"
                    "server = \"192.168.1.1\"\n"
                    "ports = [8000, 8001, 8002]\n"
                    "connection_max = 5000\n"
                    "enabled = true\n";
                const char* output = to_json(Toml, input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout(JSON.to_string());
    }
    #[test]
    fn c_api_convert_toml_to_yaml() {
        (assert_c! {
            #include "jyt.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const char* input =
                    "title = \"TOML Example\"\n"
                    "\n"
                    "[owner]\n"
                    "name = \"Tom Preston-Werner\"\n"
                    "\n"
                    "[database]\n"
                    "server = \"192.168.1.1\"\n"
                    "ports = [8000, 8001, 8002]\n"
                    "connection_max = 5000\n"
                    "enabled = true\n";
                const char* output = to_yaml(Toml, input);
                printf("%s", output);
                free((char*)output);
                return 0;
            }
        })
        .success()
        .stdout(YAML.to_string());
    }
}
