use crate::config;
use crate::utils::{test_utils, macros};

#[cfg(test)]
mod tests {
    use config::{Empty, EmptyGeneric, Property, PropertyWithDefault};

    use crate::{assert_eq_with_cleanup, assert_with_cleanup, generators::license::{License, Licenses}};
    use super::*;

    // TESTS: Empty json(no license) should return Config { license: None }
    #[test]
    fn test_read_config_empty_json() {

        let content = "{
        }";

        // Write temp file
        let file_path = test_utils::create_temp_file(content);
        
        // Call the function to read the config
        let result = config::Config::read_config(&file_path);


        
        // Print the result for debugging
        match &result {
            Ok(config) => println!("Config was read successfully: {:?}\n", config),
            Err(e) => {
                eprintln!("Failed to read config: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            },
        }

        assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n", &file_path);

        let config = result.unwrap();

        assert_eq_with_cleanup!(config.license, Option::<Property<License>>::None, &file_path);

        test_utils::remove_config_from_path(&file_path);
    }

    // TESTS: Empty json(no license) should return Config { license: None }
    #[test]
    fn test_read_config_empty_license() {

        let content = r#"{
            "license": {}
        }"#;

        // Write temp file
        let file_path = test_utils::create_temp_file(content);
        
        // Call the function to read the config
        let result = config::Config::read_config(&file_path);


        // Print the result for debugging
        match &result {
            Ok(config) => println!("Config was read successfully: {:?}\n", config),
            Err(e) => {
                eprintln!("Failed to read config: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            },
        }

        assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n");

        let config = result.unwrap_or_else(|_e| {
            // peform cleanup
            test_utils::remove_config_from_path(&file_path);
            panic!("Unable to unwrap config from the result\n");
        });

        assert_eq_with_cleanup!(config.license, Option::<Property<License>>::None, &file_path);

        test_utils::remove_config_from_path(&file_path);
    }


    // TESTS: License with only name should return license with name && default location
    #[test]
    fn test_read_config_license_only_name() {

        let content = r#"{
            "license": {
                "name": "MIT"
            }
        }"#;

        // Write temp file
        let file_path = test_utils::create_temp_file(content);
        
        // Call the function to read the config
        let result = config::Config::read_config(&file_path);


        // Print the result for debugging
        match &result {
            Ok(config) => println!("Config was read successfully: {:?}\n", config),
            Err(e) => {
                eprintln!("Failed to read config: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            },
        }

        assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n");

        let config = result.unwrap_or_else(|_e| {
            // peform cleanup
            test_utils::remove_config_from_path(&file_path);
            panic!("Unable to unwrap config from the result\n");
        });

        assert_eq_with_cleanup!(config.license, Some(Property {
            key: "license".to_string(),
            value: Some(License {
                publisher: None,
                name: Some(Property {
                    key: "name".to_string(),
                    value: Some(Licenses::MIT)
                }),
                location: Some(PropertyWithDefault {
                    key: "location".to_string(),
                    value: "root directory".to_string()
                })
            })
        }), &file_path);

        test_utils::remove_config_from_path(&file_path);
    }

    // TESTS: License with only publisher should return license with publisher && default location
    #[test]
    fn test_read_config_license_only_publisher() {

        let content = r#"{
            "license": {
                "publisher": "Meta"
            }
        }"#;

        // Write temp file
        let file_path = test_utils::create_temp_file(content);
        
        // Call the function to read the config
        let result = config::Config::read_config(&file_path);


        // Print the result for debugging
        match &result {
            Ok(config) => println!("Config was read successfully: {:?}\n", config),
            Err(e) => {
                eprintln!("Failed to read config: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            },
        }

        assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n");

        let config = result.unwrap_or_else(|_e| {
            // peform cleanup
            test_utils::remove_config_from_path(&file_path);
            panic!("Unable to unwrap config from the result\n");
        });

        assert_eq_with_cleanup!(config.license, Some(Property {
            key: "license".to_string(),
            value: Some(License {
                publisher: Some(Property {
                    key: "publisher".to_string(),
                    value: Some("Meta".to_string())
                }),
                name: None,
                location: Some(PropertyWithDefault {
                    key: "location".to_string(),
                    value: "root directory".to_string()
                })
            })
        }), &file_path);

        test_utils::remove_config_from_path(&file_path);
    }

    // TESTS: License with only location should return None
    #[test]
    fn test_read_config_license_only_location() {

        let content = r#"{
            "license": {
                "location": "custom path"
            }
        }"#;

        // Write temp file
        let file_path = test_utils::create_temp_file(content);
        
        // Call the function to read the config
        let result = config::Config::read_config(&file_path);


        // Print the result for debugging
        match &result {
            Ok(config) => println!("Config was read successfully: {:?}\n", config),
            Err(e) => {
                eprintln!("Failed to read config: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            },
        }

        assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n");

        let config = result.unwrap_or_else(|_e| {
            // peform cleanup
            test_utils::remove_config_from_path(&file_path);
            panic!("Unable to unwrap config from the result\n");
        });

        assert_eq_with_cleanup!(config.license, Option::<Property<License>>::None, &file_path);

        test_utils::remove_config_from_path(&file_path);
    }

    // TESTS: License with invalid MIT should use the Unknown and properly set the name to the provided string
    #[test]
    fn test_read_config_license_unknown_name() {

        let content = r#"{
            "license": {
                "publisher": "Meta",
                "name": "idkwhatlicenseisthis",
                "location": "custom path"
            }
        }"#;

        // Write temp file
        let file_path = test_utils::create_temp_file(content);
        
        // Call the function to read the config
        let result = config::Config::read_config(&file_path);


        // Print the result for debugging
        match &result {
            Ok(config) => println!("Config was read successfully: {:?}\n", config),
            Err(e) => {
                eprintln!("Failed to read config: {:?}\n", e);
                // cleanup
                test_utils::remove_config_from_path(&file_path);
            },
        }

        assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n");

        let config = result.unwrap_or_else(|_e| {
            // peform cleanup
            test_utils::remove_config_from_path(&file_path);
            panic!("Unable to unwrap config from the result\n");
        });

        assert_eq_with_cleanup!(config.license, Some(Property::new("license", Some(License::new(Some("Meta"), Some(Licenses::UNKNOWN), Some("custom path"))))), &file_path);

        test_utils::remove_config_from_path(&file_path);
    }

    // TESTS: should write license to javascript
    #[test]
    fn test_write_config_license_to_js() {}

    // TEST: should write license to typescript
    #[test]
    fn test_write_config_license_to_ts() {}

    // // TESTS: no publisher
    // #[test]
    // fn test_read_config_no_publisher() {

    //     let content = r#"{
    //         "license": {
    //             "name": "MIT",
    //             "location": "root directory"
    //         }
    //     }"#;

    //     // Write temp file
    //     let file_path = test_utils::create_temp_file(content);
        
    //     // Call the function to read the config
    //     let result = config::Config::read_config(&file_path);


    //     // Print the result for debugging
    //     match &result {
    //         Ok(config) => println!("Config was read successfully: {:?}\n", config),
    //         Err(e) => {
    //             eprintln!("Failed to read config: {:?}\n", e);
    //             // cleanup
    //             test_utils::remove_config_from_path(&file_path);
    //         },
    //     }

    //     assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n", &file_path);

    //     let config = result.unwrap_or_else(|_e| {
    //         // peform cleanup
    //         test_utils::remove_config_from_path(&file_path);
    //         panic!("Unable to unwrap config from the result\n");
    //     });

    //     assert_eq_with_cleanup!(config.license, Some(Property {
    //         key: "license".to_string(),
    //         value: License {
    //             publisher: None,
    //             name: Some(Property {
    //                 key: "name".to_string(),
    //                 value: Licenses::MIT
    //             }),
    //             location: Some(PropertyWithDefault {
    //                 key: "location".to_string(),
    //                 value: "root directory".to_string()
    //             })
    //         }
    //     }), &file_path);

    //     test_utils::remove_config_from_path(&file_path);
    // }

    //  // TESTS: no name
    //  #[test]
    //  fn test_read_config_no_name() {
 
    //      let content = r#"{
    //          "license": {
    //              "publisher": "Meta",
    //              "location": "root directory"
    //          }
    //      }"#;
 
    //      // Write temp file
    //      let file_path = test_utils::create_temp_file(content);
         
    //      // Call the function to read the config
    //      let result = config::Config::read_config(&file_path);
 
 
    //      // Print the result for debugging
    //      match &result {
    //          Ok(config) => println!("Config was read successfully: {:?}\n", config),
    //          Err(e) => {
    //              eprintln!("Failed to read config: {:?}\n", e);
    //              // cleanup
    //              test_utils::remove_config_from_path(&file_path);
    //          },
    //      }
 
    //      assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n", &file_path);
 
    //      let config = result.unwrap_or_else(|_e| {
    //          // peform cleanup
    //          test_utils::remove_config_from_path(&file_path);
    //          panic!("Unable to unwrap config from the result\n");
    //      });
 
    //      assert_eq_with_cleanup!(config.license, Some(Property {
    //          key: "license".to_string(),
    //          value: License {
    //             publisher: Some(Property {
    //                  key: "publisher".to_string(),
    //                  value: "Meta".to_string()
    //              }),
    //              name: None,
    //              location: Some(PropertyWithDefault {
    //                  key: "location".to_string(),
    //                  value: "root directory".to_string()
    //              })
    //          }
    //      }), &file_path);
 
    //      test_utils::remove_config_from_path(&file_path);
    //  }

    //  // TESTS: no location
    //  #[test]
    //  fn test_read_config_no_location() {
 
    //      let content = r#"{
    //          "license": {
    //              "publisher": "Meta",
    //              "name": "MIT"
    //          }
    //      }"#;
 
    //      // Write temp file
    //      let file_path = test_utils::create_temp_file(content);
         
    //      // Call the function to read the config
    //      let result = config::Config::read_config(&file_path);
 
 
    //      // Print the result for debugging
    //      match &result {
    //          Ok(config) => println!("Config was read successfully: {:?}\n", config),
    //          Err(e) => {
    //              eprintln!("Failed to read config: {:?}\n", e);
    //              // cleanup
    //              test_utils::remove_config_from_path(&file_path);
    //          },
    //      }
 
    //      assert_with_cleanup!(result.is_ok(), "Config should be read successfully\n", &file_path);
 
    //      let config = result.unwrap_or_else(|_e| {
    //          // peform cleanup
    //          test_utils::remove_config_from_path(&file_path);
    //          panic!("Unable to unwrap config from the result\n");
    //      });
 
    //      assert_eq_with_cleanup!(config.license, Some(Property {
    //          key: "license".to_string(),
    //          value: License {
    //             publisher: Some(Property {
    //                  key: "publisher".to_string(),
    //                  value: "Meta".to_string()
    //              }),
    //              name: Some(Property {
    //                 key: "name".to_string(),
    //                 value: Licenses::MIT,
    //              }),
    //              location: Some(PropertyWithDefault::on_none("location", "root directory".to_string()))
    //          }
    //      }), &file_path);
 
    //      test_utils::remove_config_from_path(&file_path);
    //  }

}