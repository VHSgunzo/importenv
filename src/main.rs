use std::env;
use std::fs::read;
use std::ffi::CString;
use std::process::exit;
use std::str::from_utf8;
use nix::unistd::execvpe;

static RED: &str = "\x1b[91m";
static RESETCOLOR: &str = "\x1b[0m";

fn error_msg(msg: String) {
    eprintln!("{}[ ERROR ]: {}{}", RED, msg, RESETCOLOR)
}

fn main() {
    let mut exec_args: Vec<String> = env::args().skip(1).collect();
    if exec_args.len() < 2 {
        error_msg(format!("Syntax: importenv <PID> <command> <command args>"));
        exit(1);
    };

    let pid = exec_args.remove(0);

    let exec_args: Vec<_> = exec_args.iter().map(|arg| 
        CString::new(arg.as_bytes()).unwrap()
    ).collect();

    let environ = format!("/proc/{}/environ", pid);
    let environ_data = read(&environ).unwrap_or_else(|err| {
        error_msg(format!("{}: {}", err, environ));
        exit(1);
    });

    let mut exec_env = Vec::new();
    for var in environ_data.split(|b| *b == 0) {
        if !var.is_empty() {
            exec_env.push(CString::new(from_utf8(var).unwrap()).unwrap())
        }
    };

    execvpe(&exec_args[0], &exec_args, &exec_env).unwrap_or_else(|err| {
        error_msg(format!("{}: {:?}", err, &exec_args[0]));
        exit(1);
    });
}
