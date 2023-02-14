use dashmap::DashMap;
use mime::Mime;
use url::Url;

use crate::{errors::PsiResult, LanguageID, LanguageRegistry, PsiError};

pub struct LanguageFileResolver {
    user_edit: DashMap<String, LanguageID>,
}

impl LanguageFileResolver {
    // normal url
    pub fn check_file_name(&self, languages: &LanguageRegistry, file_url: &Url) -> PsiResult<LanguageID> {
        // last part of url
        let file_name = match file_url.path_segments().and_then(|mut s| s.last()) {
            Some(s) => s,
            None => Err(PsiError::runtime_error(format!("File name of `{}` not valid", file_url)))?,
        };
        for language in languages.get_all_languages() {
            for support_name in language.file_names.iter() {
                if language.case_insensitive {
                    if file_name.eq_ignore_ascii_case(support_name) {
                        return Ok(language.id);
                    }
                }
                else {
                    if file_name.eq(support_name) {
                        return Ok(language.id);
                    }
                }
            }
        }
        for get_all_language in languages.get_all_languages() {
            for file_extension in get_all_language.file_extensions.iter() {
                if file_name.ends_with(file_extension) {
                    return Ok(get_all_language.id);
                }
            }
        }
        Err(PsiError::runtime_error(format!("File `{}` does not match any language registered", file_url)))
    }
    // "data:text/plain,HelloWorld"
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
