use ::std::{collections::HashSet, env, fs, path::Path};
use langram::{EnumCount, ScriptLanguage, UcdScript};

#[test]
fn count_language_models() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_dir_path = Path::new(&project_dir);
    let models_dir = project_dir_path.join("models");

    let mut slangs_count = 0;
    let mut scripts = HashSet::new();
    for lang_dir in fs::read_dir(models_dir).unwrap() {
        let lang_dir = lang_dir.unwrap();
        let lang_name = lang_dir.file_name().into_string().unwrap();
        let Some(slang) = ScriptLanguage::from_str(&lang_name) else {
            continue;
        };
        scripts.insert(UcdScript::from(slang));
        slangs_count += 1;
    }

    assert_eq!(188, slangs_count, "Change models count in Langram docs",);
    assert_eq!(
        // Excludes Common and Inherited
        158 + 2,
        UcdScript::COUNT - scripts.len(),
        "Change scripts with no models count in Langram docs",
    );
}
