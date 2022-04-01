use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Index};
use std::path::Path;
use regex::Regex;
use crate::error::Error;

pub fn preprocess(str: String, include_paths: &Vec<&str>, already_include: &Vec<String>, macros: &mut HashMap<String, String>) -> (Option<String>, Option<String>) {
    let lines: Vec<&str> = str.lines().collect();
    let mut result = String::new();

    let mut already_include = already_include.clone();

    for (i, r_line) in lines.iter().enumerate() {

        let mut line = r_line.to_string();

        // TODO: no macro expansions in strings
        for (macro_name, macro_body) in &*macros {
            let re = Regex::new(macro_name.as_str()).unwrap();
            line = re.replace_all(line.as_str(), macro_body.as_str()).to_string();
        }

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

                let mut file_locations: Vec<String> = vec![file_path.to_string()];
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

                    let (preprocessed, preprocess_error) = preprocess(file_content, include_paths, &already_include, macros);
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
                let macro_body = "(".to_owned() + args[1..].join(" ").as_str() + ")";

                macros.insert(macro_name, macro_body);

                continue;
            }

            return (None, Some(format!("unknown preprocessor directive '{}' in line {}", command, i + 1)));
        }

        result.push_str(line.as_str());
        result.push('\n');
    }

    (Some(result), None)
}