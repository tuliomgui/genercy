use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

use crate::webserver::error::TemplateError;

pub struct Templates {}

static INSTANCE: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

impl Templates {
    const INIT_PATTERN: &'static str = "{{";
    //const INIT_PATTERN_SIZE: usize = Templates::INIT_PATTERN.len();
    const END_PATTERN: &'static str = "}}";
    const END_PATTERN_SIZE: usize = Templates::END_PATTERN.len();
    const INIT_INCLUDE: &'static str = "{{@include('";
    const INIT_INCLUDE_SIZE: usize = Templates::INIT_INCLUDE.len();
    const END_INCLUDE: &'static str = "')}}";
    const END_INCLUDE_SIZE: usize = Templates::END_INCLUDE.len();
    const TEMPLATES_DIR_NAME: &'static str = "template";
    const ERROR_MSG_CLOSING_BEFORE_OPENING: &'static str = "Closing variable identifier (`}}`) appears before opening variable identifier (`{{`).";
    const ERROR_MSG_CLOSING_NOT_FOUND: &'static str = "Closing variable identifier (`}}`) not found.";
    //const ERROR_TEMPLATE_FOLDER_NOT_FOUND: &'static str = "Template folder was not found";
    const ERRO_INCLUDING_TEMPLATE: &'static str = "There was an error including a template file: ";

    pub fn mount_template(str_parts: &Vec<String>, vars_content: &HashMap<String, String>) -> Result<String, TemplateError> {
        let mut final_str: String = "".to_string();
        for partial_str in str_parts {
            if partial_str.starts_with("{{") {
                let var_name = &partial_str[2..partial_str.len()-2];
                match vars_content.get(var_name) {
                    Some(content) => { final_str.push_str(content); },
                    None => { return Err( TemplateError { message: format!("Variable content was not found on the HashMap ({})", var_name) });
                    }
                }
            } else {
                final_str.push_str(partial_str);
            }
        }
        Ok(final_str)
    }

    pub fn new() -> &'static HashMap<String, Vec<String>> {
        INSTANCE.get_or_init(|| {
            let mut tpl_map = HashMap::new();
            if let Ok(entries) = fs::read_dir(Templates::TEMPLATES_DIR_NAME) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.path().is_file() {
                            let file_name = entry.file_name().into_string().ok().unwrap();
                            match Templates::get_template_file_as_string(&entry.path().display().to_string()) {
                                Ok(string_from_file) => {
                                    match Templates::get_vector_from_string_template(string_from_file) {
                                        Ok(tpl_vector) => { tpl_map.insert(file_name, tpl_vector); }
                                        Err(_) => panic!()
                                    }
                                },
                                Err(_) => { panic!() }
                            }
                        }
                    }
                }
                return tpl_map
            }
            panic!()
        })
    }

    fn get_vector_from_string_template(string_template: String) -> Result<Vec<String>, TemplateError> {
        let mut str_vector = vec![];
        //let mut str_index = 0;
        let str_slices = Templates::get_template_slices(&string_template)?;
        let first_slice = str_slices.0;
        str_vector.push(string_template[first_slice.0..first_slice.1].to_string());
        if let None = str_slices.1 {
            return Ok(str_vector);
        }
        let second_slice = str_slices.1.unwrap();
        let var_name = string_template[second_slice.0..second_slice.1].to_string();
        if var_name.starts_with(Templates::INIT_INCLUDE) {
            let init_index = Templates::INIT_INCLUDE_SIZE;
            let end_index = var_name.len() - Templates::END_INCLUDE_SIZE;
            let new_tpl_name = &var_name[init_index..end_index];
            let file_path = format!("{}{}{}", Templates::TEMPLATES_DIR_NAME, std::path::MAIN_SEPARATOR, new_tpl_name);
            match Templates::get_template_file_as_string(&file_path) {
                Ok(new_tpm_str) => str_vector.extend(Templates::get_vector_from_string_template(new_tpm_str)?),
                Err(error) => return Err( TemplateError { message: format!("{}{}. Error: {:?}", Templates::ERRO_INCLUDING_TEMPLATE, file_path, error) })
            }
        } else {
            str_vector.push(var_name);
        }
        let third_slice = str_slices.2.unwrap();
        let recursive_vector = Templates::get_vector_from_string_template(String::from(&string_template[third_slice.0..third_slice.1]))?;
        str_vector.extend(recursive_vector);
        Ok(str_vector)
    }

    fn get_template_slices(string_template: &String) -> Result<((usize, usize), Option<(usize, usize)>, Option<(usize, usize)>), TemplateError> {
        return if let Some(init_index) = string_template.find(Templates::INIT_PATTERN) {
            if let Some(end_index) = string_template.find(Templates::END_PATTERN) {
                return if end_index < init_index {
                    Err(TemplateError { message: String::from(Templates::ERROR_MSG_CLOSING_BEFORE_OPENING) })
                } else {
                    Ok(((0, init_index), Some((init_index, end_index + Templates::END_PATTERN_SIZE)), Some((end_index + Templates::END_PATTERN_SIZE, string_template.len()))))
                }
            } else {
                Err(TemplateError { message: String::from(Templates::ERROR_MSG_CLOSING_NOT_FOUND) })
            }
        } else {
            Ok(((0, string_template.len()), None, None))
        }
    }

    fn get_template_file_as_string(file_name: &str) -> std::io::Result<String> {
        fs::read_to_string(file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_not_matching() {
        let test_str = String::from(r"Here is the first content without any pattern");
        let end_index = test_str.len();
        match Templates::get_template_slices(&test_str) {
            Ok(test_result) => {
                assert_eq!(test_result.0, (0, end_index));
                assert_eq!(test_result.1, None);
                assert_eq!(test_result.2, None);
            },
            Err(_) => { panic!() }
        }
    }

    #[test]
    fn test_pattern_matching_empty_left_right() {
        let test_str = format!("{}pattern{}", Templates::INIT_PATTERN, Templates::END_PATTERN);
        let test_str_len = test_str.len();
        match Templates::get_template_slices(&test_str) {
            Ok(test_result) => {
                assert_eq!(test_result.0, (0, 0));
                assert_eq!(test_result.1, Some((0, test_str_len)));
                assert_eq!(test_result.2, Some((test_str_len, test_str_len)));
            },
            Err(_) => { panic!() }
        }
    }

    #[test]
    fn test_pattern_matching_empty_right() {
        let test_str = format!(" {}pattern{}", Templates::INIT_PATTERN, Templates::END_PATTERN);
        let test_str_len = test_str.len();
        match Templates::get_template_slices(&test_str) {
            Ok(test_result) => {
                assert_eq!(test_result.0, (0, 1));
                assert_eq!(test_result.1, Some((1, test_str_len)));
                assert_eq!(test_result.2, Some((test_str_len, test_str_len)));
            },
            Err(_) => { panic!() }
        }
    }

    #[test]
    fn test_pattern_matching_empty_left() {
        let test_str = format!("{}pattern{} ", Templates::INIT_PATTERN, Templates::END_PATTERN);
        let test_str_len = test_str.len();
        match Templates::get_template_slices(&test_str) {
            Ok(test_result) => {
                assert_eq!(test_result.0, (0, 0));
                assert_eq!(test_result.1, Some((0, test_str_len - 1)));
                assert_eq!(test_result.2, Some((test_str_len - 1, test_str_len)));
            },
            Err(_) => { panic!() }
        }
    }

    #[test]
    fn test_pattern_matching_non_empty() {
        let test_str = format!(r"Here is the first content {}pattenr{} and the rest of the content", Templates::INIT_PATTERN, Templates::END_PATTERN);
        match Templates::get_template_slices(&test_str) {
            Ok(test_result) => {
                assert_eq!(test_result.0, (0, 26));
                assert_eq!(test_result.1, Some((26, 37)));
                assert_eq!(test_result.2, Some((37, 65)));
            },
            Err(_) => { panic!() }
        }
    }

    #[test]
    fn test_pattern_matching_result_string_empty_left() {
        let test_str = format!(r"{}pattern{} right.", Templates::INIT_PATTERN, Templates::END_PATTERN);
        match Templates::get_template_slices(&test_str.clone()) {
            Ok(test_result) => {
                assert_eq!(test_str[test_result.0.0..test_result.0.1], *"");
                let second_slice = test_result.1.unwrap();
                assert_eq!(test_str[second_slice.0..second_slice.1], *"{{pattern}}");
                let third_slice = test_result.2.unwrap();
                assert_eq!(test_str[third_slice.0..third_slice.1], *" right.");
            },
            Err(_) => { panic!( )}
        }
    }

    #[test]
    fn test_pattern_matching_result_string_empty_right() {
        let test_str = format!(r"Left {}pattern{}", Templates::INIT_PATTERN, Templates::END_PATTERN);
        match Templates::get_template_slices(&test_str.clone()) {
            Ok(test_result) => {
                assert_eq!(test_str[test_result.0.0..test_result.0.1], *"Left ");
                let second_slice = test_result.1.unwrap();
                assert_eq!(test_str[second_slice.0..second_slice.1], *"{{pattern}}");
                let third_slice = test_result.2.unwrap();
                assert_eq!(test_str[third_slice.0..third_slice.1], *"");
            },
            Err(_) => { panic!( )}
        }
    }

    #[test]
    fn test_pattern_matching_result_string_non_empty() {
        let test_str = format!(r"Left {}pattern{} right.", Templates::INIT_PATTERN, Templates::END_PATTERN);
        match Templates::get_template_slices(&test_str.clone()) {
            Ok(test_result) => {
                assert_eq!(test_str[test_result.0.0..test_result.0.1], *"Left ");
                let second_slice = test_result.1.unwrap();
                assert_eq!(test_str[second_slice.0..second_slice.1], *"{{pattern}}");
                let third_slice = test_result.2.unwrap();
                assert_eq!(test_str[third_slice.0..third_slice.1], *" right.");
            },
            Err(_) => { panic!( )}
        }
    }

    #[test]
    fn test_pattern_matching_error_closing_not_found() {
        let test_str = format!(r"No closing {}pattenr", Templates::INIT_PATTERN);
        let test_result = Templates::get_template_slices(&test_str);
        assert!(test_result.is_err());
        if let Err(error) = test_result {
            assert_eq!(error.message, Templates::ERROR_MSG_CLOSING_NOT_FOUND);
        }
    }

    #[test]
    fn test_pattern_matching_error_closing_before_opening() {
        let test_str = format!(r"No closing {}pattenr{}", Templates::END_PATTERN, Templates::INIT_PATTERN);
        let test_result = Templates::get_template_slices(&test_str);
        assert!(test_result.is_err());
        if let Err(error) = test_result {
            assert_eq!(error.message, Templates::ERROR_MSG_CLOSING_BEFORE_OPENING);
        }
    }

    #[test]
    fn test_get_vector_from_string_zero_variable() {
        let test_str = String::from("This is a test String with no variables");
        let test_result = Templates::get_vector_from_string_template(test_str.clone()).unwrap();
        assert_eq!(test_result.len(), 1);
        assert_eq!(test_result[0], test_str);
    }

    #[test]
    fn test_get_vector_from_string_one_variable() {
        let str_1 = "This is a test String with ";
        let str_2 = "one";
        let str_3 = " variable";
        let test_str = format!("{}{}{}{}{}", str_1, Templates::INIT_PATTERN, str_2, Templates::END_PATTERN, str_3);
        let test_result = Templates::get_vector_from_string_template(test_str.clone()).unwrap();
        assert_eq!(test_result.len(), 3);
        assert_eq!(test_result[0], str_1);
        assert_eq!(test_result[1], "{{one}}");
        assert_eq!(test_result[2], str_3);
    }

    #[test]
    fn test_get_vector_from_string_two_variable() {
        let str_1 = "This is a test String with ";
        let str_2 = "first";
        let str_3 = " and ";
        let str_4 = "second";
        let str_5 = " variables ";
        let test_str = format!("{}{}{}{}{}{}{}{}{}", str_1,
                               Templates::INIT_PATTERN, str_2, Templates::END_PATTERN, str_3,
                               Templates::INIT_PATTERN, str_4, Templates::END_PATTERN, str_5);
        let test_result = Templates::get_vector_from_string_template(test_str.clone()).unwrap();
        assert_eq!(test_result.len(), 5);
        assert_eq!(test_result[0], str_1);
        assert_eq!(test_result[1], "{{first}}");
        assert_eq!(test_result[2], str_3);
        assert_eq!(test_result[3], "{{second}}");
        assert_eq!(test_result[4], str_5);
    }

    #[test]
    fn test_mount_template() {
        let test_templates = Templates::new();
        let str_vector = test_templates.get("test_1.html").unwrap();
        let expected_str = "This is the test file";
        let mut var_content_map = HashMap::new();
        var_content_map.insert(String::from("name"), String::from("test"));
        let result_str = Templates::mount_template(str_vector, &var_content_map).unwrap();
        assert_eq!(&result_str, expected_str);
    }

    #[test]
    fn test_mount_template_with_include() {
        let test_templates = Templates::new();
        let str_vector = test_templates.get("test_include.html").unwrap();
        let expected_str = "This is the test file";
        let mut var_content_map = HashMap::new();
        var_content_map.insert(String::from("name"), String::from("test"));
        let result_str = Templates::mount_template(str_vector, &var_content_map).unwrap();
        assert_eq!(&result_str, expected_str);
    }
}