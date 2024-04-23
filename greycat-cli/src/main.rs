use std::{collections::HashMap, fs::File, time::Instant};

use greycat_sdk::prelude::*;
use reqwest::blocking::*;

fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let bytes = client
        .post("http://localhost:8080/runtime::Runtime::abi")
        .header("accept", "application/octet-stream")
        .send()?
        .bytes()?;

    let abi = AbiBuilder::new().build(&*bytes)?;

    serde_json::to_writer(File::create("abi.json")?, &abi)?;

    let args = Value::Array(vec![
        Value::Int(42),
        Value::Symbol(
            abi.get_symbol("hello")
                .expect("symbol 'hello' is supposed to be known"),
        ),
        #[allow(clippy::approx_constant)]
        Value::Float(3.14.into()),
        Value::Char('d'),
        Value::Bool(false),
        Value::String(String::from("Wesh alors bien ou bien?")),
        Value::Bool(true),
        Value::Obj(GcObject::new(
            abi.get_type_by_fqn("project::Stats")
                .expect("test project.gcl is expected to define a type 'project::Stats'"),
            Some([Value::Int(42)]),
        )),
        Value::Obj(GcObject::new(
            abi.get_type_by_fqn("project::PersonGroup")
                .expect("test project.gcl is expected to define a type 'project::PersonGroup'"),
            Some([
                // name: String
                Value::Symbol(
                    abi.get_symbol("John")
                        .expect("symbol 'John' is supposed to be known"),
                ),
                // admin: Person
                Value::Obj(GcObject::new(
                    abi.get_type_by_fqn("project::Person")
                        .expect("test project.gcl is expected to define a type 'project::Person'"),
                    Some([
                        // name: String
                        Value::String("Leiko".into()),
                        // age: int
                        Value::Int(1337),
                    ]),
                )),
                // members: Array<Person>
                Value::Array(vec![]),
                // recursion: PersonGroup?
                Value::Null,
            ]),
        )),
    ]);

    let mut body: Vec<u8> = Vec::new();
    let mut n = abi.headers.write_to(&mut body, &abi)?;
    n += args.write_to(&mut body, &abi)?;
    eprintln!("payload of {n} bytes");
    assert_eq!(body.len(), n);

    let res = client
        .post("http://localhost:8080/project::anything")
        .body(body)
        .header("content-type", "application/octet-stream")
        .header("accept", "application/octet-stream")
        .send()?;

    let bytes = res.bytes()?;
    let mut bytes = &bytes[..];

    let _ = bytes.read_request_headers()?;
    let result = bytes.read_value(&abi)?;
    assert!(bytes.is_empty());
    assert_eq!(args, result);
    eprintln!("{result:#?}");

    Ok(())
}

// fn main() -> anyhow::Result<()> {
//     let client = Client::new();

//     let bytes = client
//         .post("http://localhost:8080/runtime::Runtime::abi")
//         .header("accept", "application/octet-stream")
//         .send()?
//         .bytes()?;
//     let mut bytes = &bytes[..];

//     let headers = bytes.read_abi_headers()?;
//     let symbols = bytes.read_abi_symbols()?;
//     let types = bytes.read_abi_types(&symbols)?;
//     let functions = bytes.read_abi_functions(&symbols, &types)?;
//     let abi = Abi {
//         headers,
//         symbols: &symbols,
//         types: types.clone(),
//         functions: functions.clone(),
//     };
//     serde_json::to_writer(File::create("abi.json")?, &abi)?;

//     // _run_read_endpoints(&client, &abi)?;
//     let value = _run(&client, &abi, "project::loadProblem")?;
//     serde_json::to_writer_pretty(std::io::stdout(), &value)?;

//     Ok(())
// }

fn _run_records() -> anyhow::Result<()> {
    let bytes = std::fs::read("gcdata/store/abi").unwrap();
    let abi = Abi::new(&*bytes, None)?;

    let mut bytes = &std::fs::read("gcdata/files/records.gcb").unwrap()[..];
    let _headers = bytes.read_request_headers()?;
    let start = Instant::now();
    loop {
        if bytes.is_empty() {
            break;
        }
        let _value = bytes.read_value(&abi)?;
    }
    println!("Took {:?}", Instant::now() - start);

    Ok(())
}

fn _run_read_endpoints(client: &Client, abi: &Abi) -> anyhow::Result<()> {
    let mut requests = HashMap::new();

    let read_id = abi.get_symbol_id("read").unwrap_or(0);

    for f in &*abi.functions {
        if f.module != read_id {
            continue;
        }
        let fqn = f.fqn();
        let value = _run(client, abi, &fqn)?;
        requests.insert(fqn, value);
    }

    serde_json::to_writer_pretty(std::io::stdout(), &requests)?;

    Ok(())
}

fn _run<'abi>(client: &Client, abi: &'abi Abi, method: &str) -> anyhow::Result<Value<'abi>> {
    let bytes = client
        .post(format!("http://localhost:8080/{method}"))
        .header("accept", "application/octet-stream")
        .send()?
        .bytes()?;
    let mut bytes = &bytes[..];

    let _headers = bytes.read_request_headers()?;
    let value = bytes.read_value(abi)?;
    assert!(bytes.is_empty());

    Ok(value)
}
