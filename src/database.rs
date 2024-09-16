use redb::{Database, Error, ReadableTable, TableDefinition};

const TABLE: TableDefinition<u32, f64> = TableDefinition::new("data");

fn write(uid: u32, value: f64) -> Result<(), Error> {
    let file = tempfile::NamedTempFile::new().unwrap();
    let db = Database::create(file.path())?;
    let write_txn = db.begin_write()?;
    
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert(uid, value)?;
    }
    
    write_txn.commit()?;

    Ok(())
}

fn read(uid: u32) -> Result<f64, Error> {

    let file = tempfile::NamedTempFile::new().unwrap();
    let db = Database::create(file.path())?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;

    let ae = table.get(uid)?.unwrap().value();

    Ok(ae)
}