use std::io;
use std::process::ExitCode;

use std::path::PathBuf;

use rs_jsonl2redb::tablename2stdin2redb2path;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_table_name() -> impl Fn() -> String {
    io_envkey2val("ENV_TABLE_NAME")
}

fn io_redb_path() -> impl Fn() -> String {
    io_envkey2val("ENV_OUTPUT_REDB_PATH")
}

fn io_stdin2path2redb() -> impl FnOnce(PathBuf) -> Result<(), io::Error> {
    let tabname: String = io_table_name()();
    tablename2stdin2redb2path(tabname)
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let redb_path: String = io_redb_path()();
        let stdin2redb = io_stdin2path2redb();
        stdin2redb(redb_path.into())
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");

        let table_name: String = io_table_name()();
        let redb_path: String = io_redb_path()();

        eprintln!("ENV_TABLE_NAME: {table_name}");
        eprintln!("ENV_OUTPUT_REDB_PATH: {redb_path}");

        ExitCode::FAILURE
    })
}
