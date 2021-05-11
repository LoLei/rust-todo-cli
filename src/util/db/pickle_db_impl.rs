use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn init_pickle_db(filename: &str) -> PickleDb {
    // Try loading existing DB from a file
    let attempt_db_load = PickleDb::load(
        filename,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    // If no existing db, create new one
    let db = if let Ok(d) = attempt_db_load {
        println!("Using existing db at {}", filename);
        d
    } else {
        println!("Creating new db at {}", filename);
        PickleDb::new(
            filename,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    };

    db
}
