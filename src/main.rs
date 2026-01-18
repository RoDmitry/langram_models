use ::std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Instant,
};
use langram::{bin_storage::BinStorage, ScriptLanguage, UcdScript};
use langram_train::file_model::dir_into_model;

const THREADS: usize = 8;

fn main() {
    let start = Instant::now();

    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_dir_path = Path::new(&project_dir);
    let models_dir = project_dir_path.join("models");

    // let out_dir = env::var("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_dir);

    let pool = threadpool::ThreadPool::new(THREADS);
    let bin_storage = Arc::new(Mutex::new(BinStorage::default()));

    let mut scripts_langs: HashMap<UcdScript, Vec<(ScriptLanguage, PathBuf)>> = HashMap::new();
    for lang_dir in fs::read_dir(models_dir).unwrap() {
        let lang_dir = lang_dir.unwrap();
        let lang_dir_path = lang_dir.path();
        let lang_name = lang_dir.file_name().into_string().unwrap();

        let Some(slang) = ScriptLanguage::from_str(&lang_name) else {
            println!("{} Not found", lang_name);
            continue;
        };

        let entry = scripts_langs.entry(UcdScript::from(slang)).or_default();
        entry.push((slang, lang_dir_path));
    }

    for (script, langs) in scripts_langs {
        if langs.len() == 1 {
            let mut slangs = ScriptLanguage::all_with_script(script).to_vec();
            slangs.sort();
            let top_lang = *slangs.first().unwrap();
            // skips single top lang
            if langs.first().unwrap().0 == top_lang {
                println!("Skipped {}", top_lang.into_str());
                continue;
            }
        }

        for (l, path) in langs {
            let bin_storage_clone = bin_storage.clone();
            pool.execute(move || {
                if let Some(model) = dir_into_model(path).unwrap() {
                    let mut storage = bin_storage_clone.lock().unwrap();
                    println!("{}", l.into_str());
                    storage.add(l, model);
                }
            });
        }
    }

    pool.join();

    let mut bin_storage = bin_storage.lock().unwrap();
    println!("Finalizing...");
    bin_storage.finalize();

    println!("To bytes...");
    let bytes = bin_storage.to_bytes().unwrap();

    println!("Saving...");
    let compiled_models_path = project_dir_path.join(BinStorage::FILE_NAME);
    fs::write(compiled_models_path, bytes).unwrap();

    println!("built in {:.2} sec", start.elapsed().as_secs_f64());
}
