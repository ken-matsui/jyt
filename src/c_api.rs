use crate::ext;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// # Safety
///
/// The returned char pointer should be freed by the `cstring_free` function.
/// For detailed usage in any language, please see:
/// https://jakegoulding.com/rust-ffi-omnibus/string_return/
#[no_mangle]
pub unsafe extern "C" fn to_json(from: ext::Ext, input: *const c_char) -> *mut c_char {
    let input = CStr::from_ptr(input);
    let deserialized = ext::deserialize::<ext::json::Value>(from, input.to_str().unwrap());
    let serialized = CString::new(ext::json::serialize(&deserialized.unwrap()).unwrap()).unwrap();
    serialized.into_raw()
}

/// # Safety
///
/// The returned char pointer should be freed by the `cstring_free` function.
/// For detailed usage in any language, please see:
/// https://jakegoulding.com/rust-ffi-omnibus/string_return/
#[no_mangle]
pub unsafe extern "C" fn to_yaml(from: ext::Ext, input: *const c_char) -> *mut c_char {
    let input = CStr::from_ptr(input);
    let deserialized = ext::deserialize::<ext::yaml::Value>(from, input.to_str().unwrap());
    let serialized = CString::new(ext::yaml::serialize(&deserialized.unwrap()).unwrap()).unwrap();
    serialized.into_raw()
}

/// # Safety
///
/// The returned char pointer should be freed by the `cstring_free` function.
/// For detailed usage in any language, please see:
/// https://jakegoulding.com/rust-ffi-omnibus/string_return/
#[no_mangle]
pub unsafe extern "C" fn to_toml(from: ext::Ext, input: *const c_char) -> *mut c_char {
    let input = CStr::from_ptr(input);
    let deserialized = ext::deserialize::<ext::toml::Value>(from, input.to_str().unwrap());
    let serialized = CString::new(ext::toml::serialize(&deserialized.unwrap()).unwrap()).unwrap();
    serialized.into_raw()
}

/// # Safety
///
/// This function is for freeing a pointer of the argument.
/// The pointer should be allocated with `CString` in the Rust world.
#[no_mangle]
pub unsafe extern "C" fn cstring_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }

    // retake pointer to free memory
    let _ = CString::from_raw(s);
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

    pub(crate) static YAML: &str = r#"title: TOML Example
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
                char* output = to_yaml(Json, input);
                printf("%s", output);
                cstring_free(output);
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
                char* output = to_toml(Json, input);
                printf("%s", output);
                cstring_free(output);
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
                    "title: TOML Example\n"
                    "owner:\n"
                    "  name: Tom Preston-Werner\n"
                    "database:\n"
                    "  server: 192.168.1.1\n"
                    "  ports:\n"
                    "  - 8000\n"
                    "  - 8001\n"
                    "  - 8002\n"
                    "  connection_max: 5000\n"
                    "  enabled: true\n";
                char* output = to_json(Yaml, input);
                printf("%s", output);
                cstring_free(output);
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
                    "title: TOML Example\n"
                    "owner:\n"
                    "  name: Tom Preston-Werner\n"
                    "database:\n"
                    "  server: 192.168.1.1\n"
                    "  ports:\n"
                    "  - 8000\n"
                    "  - 8001\n"
                    "  - 8002\n"
                    "  connection_max: 5000\n"
                    "  enabled: true\n";
                char* output = to_toml(Yaml, input);
                printf("%s", output);
                cstring_free(output);
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
                char* output = to_json(Toml, input);
                printf("%s", output);
                cstring_free(output);
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
                char* output = to_yaml(Toml, input);
                printf("%s", output);
                cstring_free(output);
                return 0;
            }
        })
        .success()
        .stdout(YAML.to_string());
    }
}
