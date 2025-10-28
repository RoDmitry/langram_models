use ::std::{
    env, fs,
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};
use langram::{bin_storage::BinStorage, file_model::dir_into_model};

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

    for lang_dir in fs::read_dir(models_dir).unwrap() {
        let lang_dir = lang_dir.unwrap();
        let lang_dir_path = lang_dir.path();
        let lang_name = lang_dir.file_name();

        let bin_storage_clone = bin_storage.clone();
        pool.execute(move || {
            if let Some(model) = dir_into_model(lang_dir_path).unwrap() {
                let mut storage = bin_storage_clone.lock().unwrap();
                storage.add(lang_name.into_string().unwrap(), model);
            }
        });
    }

    pool.join();

    let mut bin_storage = bin_storage.lock().unwrap();
    bin_storage.reorder();

    let compiled_models_path = project_dir_path.join(BinStorage::FILE_NAME);
    fs::write(compiled_models_path, bin_storage.to_bytes().unwrap()).unwrap();

    println!("built in {:.2} sec", start.elapsed().as_secs_f64());
}
