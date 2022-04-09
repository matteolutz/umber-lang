use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Index};
use std::path::Path;
use regex::Regex;
use crate::error::Error;

pub fn preprocess(current_file_path: &str, str: String, include_paths: &Vec<&str>, already_include: &mut Vec<String>, macros: &mut HashMap<String, String>) -> (Option<String>, Option<String>) {
    let lines: Vec<&str> = str.lines().collect();
    let mut result = String::new();

    for (i, r_line) in lines.iter().enumerate() {

        let mut line = r_line.to_string();

        if line.starts_with('#') {

            // TODO: refactor this, change comment syntax to be //
            if line.strip_prefix('#').unwrap().starts_with(' ') {
                continue;
            }

            let trimmed = line.strip_prefix('#').unwrap().trim();
            if trimmed.len() == 0 {
                return (None, Some(format!("empty preprocessor directive in line {}", i + 1)));
            }

            let mut args: Vec<&str> = trimmed.split(" ").collect();
            if args.len() == 0 {
                return (None, Some(format!("empty preprocessor directive in line {}", i + 1)));
            }

            let command = args.first().unwrap().to_string();
            args.remove(0);

            if command == "include" {
                let file_path = args.join(" ").to_string();

                let mut file_locations: Vec<String> = vec![Path::new(current_file_path).parent().unwrap().join(file_path.to_string()).to_str().unwrap().to_string()];
                for path in include_paths.iter() {
                    file_locations.push(Path::new(path).join(&file_path).to_str().unwrap().to_string());
                }

                let mut success = false;
                for loc in file_locations.iter() {
                    if already_include.contains(loc) {
                        success = true;
                        break;
                    }

                    let mut file_content_option = fs::read_to_string(loc);
                    if let Err(e) = file_content_option {
                        continue;
                    }

                    let file_content = file_content_option.unwrap();

                    let (preprocessed, preprocess_error) = preprocess(loc,file_content, include_paths, already_include, macros);
                    if let Some(error) = preprocess_error {
                        return (None, Some(format!("error preprocessing included file {}: {}", loc, error)));
                    }

                    result.push_str(preprocessed.unwrap().as_str());
                    result.push_str("\n");

                    already_include.push(loc.to_string());

                    success = true;
                    break;
                };

                if !success {
                    return (None, Some(format!("could not find included file {}", file_path)));
                }

                continue;
            }

            if command == "macro" {
                if args.len() < 2 {
                    return (None, Some(format!("macro directive requires at least 2 arguments in line {}", i + 1)));
                }

                let macro_name = args[0].to_string();
                let mut macro_body = "(".to_owned() + args[1..].join(" ").as_str() + ")";

                for (o_macro_name, o_macro_body) in &*macros {
                    let re = Regex::new(o_macro_name.as_str()).expect(format!("Invalid regex: '{}': \"{}\", line: {}\n{}", o_macro_name, o_macro_body, i+1, r_line).as_str());
                    macro_body = re.replace_all(macro_body.as_str(), o_macro_body.as_str()).to_string();
                }

                macros.insert(macro_name, macro_body);

                continue;
            }

            return (None, Some(format!("unknown preprocessor directive '{}' in line {}", command, i + 1)));
        }

        for (macro_name, macro_body) in &*macros {
            let re = Regex::new(macro_name.as_str()).expect(format!("Invalid regex: '{}': \"{}\", line: {}\n{}", macro_name, macro_body, i+1, r_line).as_str());
            line = re.replace_all(line.as_str(), macro_body.as_str()).to_string();
        }

        result.push_str(line.as_str());
        result.push('\n');
    }

    (Some(result), None)
}