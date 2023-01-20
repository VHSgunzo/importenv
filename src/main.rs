extern crate nix;
extern crate chrono;
use std::env;
use chrono::Local;
use std::io::Read;
use std::path::Path;
use std::ffi::CString;
use std::process::exit;
use std::str::from_utf8;
use nix::unistd::execvpe;
use std::fs::OpenOptions;

static RED: &str = "\x1b[91m";
static RESETCOLOR: &str = "\x1b[0m";

pub fn error_msg(msg: &str) {
    let date = Local::now().format("%Y.%m.%d %H:%M:%S");
    eprintln!("{}[ ERROR ][{}]: {}{}", RED, date, msg, RESETCOLOR);
}

fn main() {
    let mut exec_args: Vec<String> = env::args().collect();
    if exec_args.len() < 3 {
        error_msg(&format!("Syntax: {} <PID> <command> <command args>", exec_args[0]));
        exit(1);
    };
    let pid = exec_args.remove(1);
    exec_args.remove(0);
    let exec_prog = CString::new(exec_args[0].clone()).unwrap();
    let exec_prog = exec_prog.as_c_str();
    let exec_args: Vec<_> = exec_args.iter()
        .map(|arg| CString::new(arg.as_bytes()).unwrap()).collect();
    let environ = format!("/proc/{}/environ", pid);
    let environ_file_path = Path::new(&environ);
    let mut environ_file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(environ_file_path)
        .unwrap_or_else(|err| {
            error_msg(&format!("{}: \"{}\"", err.to_string(), environ));
            exit(1);
    });
    let mut environ_file_data = Vec::new();
    environ_file.read_to_end(&mut environ_file_data).unwrap();
    let mut exec_env = Vec::new();
    for var in environ_file_data.split(|b| *b == 0) {
        let var_str = from_utf8(var).unwrap();
        if ! var_str.is_empty() {
            exec_env.push(CString::new(var_str).unwrap());
        };
    };
    if let Err(err) = execvpe(exec_prog, &exec_args, &exec_env) {
        error_msg(&format!("{}: {:?}", err.to_string(), exec_prog));
        exit(1);
    };
}
