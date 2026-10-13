use std::io;

use std::path::Path;
use std::path::PathBuf;

use io::BufRead;

use redb::WriteTransaction;

use redb::Database;

use redb::Table;
use redb::TableDefinition;

pub fn brdr2redb<R>(
    mut brdr: R,
    wtx: WriteTransaction,
    sep: u8,
    sch: TableDefinition<&str, &str>,
) -> Result<(), io::Error>
where
    R: BufRead,
{
    let mut buf: Vec<u8> = vec![];

    let mut tab: Table<&str, &str> = wtx.open_table(sch).map_err(io::Error::other)?;
    loop {
        buf.clear();
        let read_cnt: usize = brdr.read_until(sep, &mut buf)?;
        if 0 == read_cnt {
            break;
        }

        let nolf: &[u8] = buf.strip_suffix(&[sep]).unwrap_or(&buf);

        let parsed: (&str, &str) = serde_json::from_slice::<(&str, &str)>(nolf)?;
        let (key, val) = parsed;
        tab.insert(key, val).map_err(io::Error::other)?;
    }
    drop(tab);

    wtx.commit().map_err(io::Error::other)
}

pub fn brdr2redb2path<R, P>(
    brdr: R,
    redb_path: P,
    sep: u8,
    table_name: &str,
) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    R: BufRead,
{
    let sch: TableDefinition<&str, &str> = TableDefinition::new(table_name);
    let db: Database = Database::create(redb_path).map_err(io::Error::other)?;
    let wtx: WriteTransaction = db.begin_write().map_err(io::Error::other)?;
    brdr2redb(brdr, wtx, sep, sch)
}

pub fn stdin2redb2path<P>(redb_path: P, sep: u8, table_name: &str) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    brdr2redb2path(io::stdin().lock(), redb_path, sep, table_name)
}

pub fn stdin2redb2path_default<P>(redb_path: P, table_name: &str) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    stdin2redb2path(redb_path, b'\n', table_name)
}

pub fn tablename2stdin2redb2path(
    table_name: String,
) -> impl FnOnce(PathBuf) -> Result<(), io::Error> {
    move |redb_path: PathBuf| stdin2redb2path_default(redb_path, &table_name)
}
