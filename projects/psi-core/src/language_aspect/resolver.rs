use std::{ffi::OsStr, path::Path};

use mime::Mime;
use url::Url;

use crate::{errors::PsiResult, LanguageID, LanguageRegistry, PsiError};

pub struct LanguageFileResolver {}

impl LanguageFileResolver {
    pub fn check_file_name(&self, languages: &LanguageRegistry, file_url: &Url) -> PsiResult<LanguageID> {
        let file = file_url.to_file_path()?.file_name().and_then(OsStr::to_str);
        for language in languages.get_all_languages() {
            for file_name in language.file_names.iter() {
                match file_url.file_name().and_then(|s| s.to_str()) {
                    Some(s) if file_name == s => return Ok(language.id),
                    Some(_) => continue,
                    None => Err(PsiError::runtime_error(format!("File name of `{}` not valid", file_url.display())))?,
                }
            }
        }
        for get_all_language in languages.get_all_languages() {
            for file_extension in get_all_language.file_extensions.iter() {
                match file_url.file_name().and_then(|s| s.to_str()) {
                    Some(s) if s.ends_with(file_extension) => return Ok(get_all_language.id),
                    // The validity of the filename has been checked above
                    _ => continue,
                }
            }
        }
        Err(PsiError::runtime_error(format!("File `{}` does not match any language registered", file_url.display())))
    }
    pub fn check_file_mime(&self, languages: &LanguageRegistry, mime: &Mime) -> PsiResult<LanguageID> {
        for language in languages.get_all_languages() {
            for file_mime in language.file_mimes.iter() {
                if file_mime == mime {
                    return Ok(language.id);
                }
            }
        }
        Err(PsiError::runtime_error(format!("Mime type `{}` does not match any language registered", mime)))
    }
}
