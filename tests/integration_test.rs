use rust_todo_cli;
use rust_todo_cli::{Operation, RtcError, RunReturn};

mod common;

#[test]
fn test_happy_path() -> Result<(), RtcError> {
    setup_db_file();

    // Add
    do_op_and_assert(Operation::Add, vec!["my todo item 1"], Assertion::Number(1))?;
    do_op_and_assert(Operation::Add, vec!["my todo item 2"], Assertion::Number(2))?;
    do_op_and_assert(Operation::Add, vec!["my todo item 3"], Assertion::Number(3))?;
    do_op_and_assert(Operation::Add, vec!["my todo item 4"], Assertion::Number(4))?;

    // Get all
    do_op_and_assert(Operation::GetAll, vec![], Assertion::Number(4))?;

    // Delete
    do_op_and_assert(Operation::Delete, vec!["2"], Assertion::Boolean(true))?;

    // Filter by name
    do_op_and_assert(
        Operation::Filter,
        vec!["name=my todo item"],
        Assertion::Number(3),
    )?;
    do_op_and_assert(Operation::Filter, vec!["name=item 3"], Assertion::Number(1))?;
    do_op_and_assert(
        Operation::Filter,
        vec!["name=this item does not exist"],
        Assertion::Number(0),
    )?;

    // Update
    do_op_and_assert(
        Operation::Update,
        vec!["3", "done"],
        Assertion::Boolean(true),
    )?;

    // Filter by other criteria
    do_op_and_assert(Operation::Filter, vec!["status=done"], Assertion::Number(1))?;
    do_op_and_assert(Operation::Filter, vec!["status=open"], Assertion::Number(2))?;

    Ok(())
}

fn setup_db_file() {
    match common::remove_db() {
        Ok(_) => println!("db removed"),
        Err(_) => println!("no db to remove"),
    }
}

/// Poor man's union of usize | bool
#[derive(Debug)]
enum Assertion {
    Number(usize),
    Boolean(bool),
}

fn do_op_and_assert(
    operation: Operation,
    args: Vec<&str>,
    assertion: Assertion,
) -> Result<(), RtcError> {
    let config = common::create_config(
        operation,
        args.into_iter().map(|it| it.to_string()).collect(),
    );

    let run_result = rust_todo_cli::run(&config)?;

    match run_result {
        RunReturn::Addition(res) => assert_eq!(
            res,
            match assertion {
                Assertion::Number(x) => x,
                _ => panic!("Wrong assertion type"),
            }
        ),
        RunReturn::Deletion(res) => assert_eq!(
            res,
            match assertion {
                Assertion::Boolean(x) => x,
                _ => panic!("Wrong assertion type"),
            }
        ),
        RunReturn::Update(res) => assert_eq!(
            res,
            match assertion {
                Assertion::Boolean(x) => x,
                _ => panic!("Wrong assertion type"),
            }
        ),
        RunReturn::Filter(res) => assert_eq!(
            res.len(),
            match assertion {
                Assertion::Number(x) => x,
                _ => panic!("Wrong assertion type"),
            }
        ),
        RunReturn::GetAll(res) => assert_eq!(
            res.len(),
            match assertion {
                Assertion::Number(x) => x,
                _ => panic!("Wrong assertion type"),
            }
        ),
    }

    Ok(())
}
