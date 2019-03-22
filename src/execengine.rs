use nix;

use nix::sys::wait::waitpid;
use nix::unistd::{alarm, close, fork, pipe, read, write, ForkResult};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::os::unix::process::ExitStatusExt;
use std::process::{exit, Command, Stdio};
use std::{env, fmt, format, fs, str};

use crate::config::{ProgConfig, SeedConfig, Stat};

pub fn exec_fuzz(seed_config: &mut SeedConfig, prog_config: &ProgConfig) {
    let fd_d = pipe().unwrap();
    let fd_c = pipe().unwrap();
    //let args: Vec<String> = env::args().collect();
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            //eprintln!("Inside parent ,child pid is {}", child);
            let mut arr: [u8; 1] = [0; 1];
            close(fd_d.1).unwrap();
            close(fd_c.1).unwrap();

            /**TODO
            Implement timeout
            **/
            let mut len = [1, 1];
            let mut data: Vec<u8> = Vec::new();
            let mut control: Vec<u8> = Vec::new();
            waitpid(child, None);
            loop {
                if len[0] == 0 {
                    break;
                }
                len[0] = read(fd_d.0, &mut arr).unwrap();
                data.push(arr[0]);
            }

            loop {
                if len[1] == 0 {
                    break;
                }
                len[1] = read(fd_c.0, &mut arr).unwrap();
                control.push(arr[0]);
            }

            if control[0] != 0 && data.len() > 1 {
                seed_config.exit_stat = Stat::CRASH;
                let c: &[u8] = data.as_slice();
                let s: &[u8] = seed_config.seed.as_slice();
                fs::write(
                    format!("{}/Crash/{}", prog_config.outputdir, seed_config.input),
                    s,
                )
                .unwrap();
                fs::write(
                    format!("{}/Crash/{}", prog_config.outputdir, seed_config.output),
                    c,
                )
                .unwrap();
            }

            //println!("Data {} len {}  \nControl {}", String::from_utf8(data.clone()).unwrap(),data.len(),control[0]);
            //println!("Fuzzer Status\n {} {} [2J",prog_config);
            close(fd_d.0).unwrap();
            close(fd_c.0).unwrap();
        }

        Ok(ForkResult::Child) => {
            close(fd_d.0);
            close(fd_c.0);
            // eprintln!("Inside Child");
            let mut args: Vec<String> = Vec::new();
            args.push(prog_config.inputpath.clone());
            //for i in seed_config.seed.clone(){//.into_iter().map(|s| s);

            unsafe {
                let output = Command::new(&args[0])
                    .args(&args[1..2])
                    .stdout(Stdio::null()) //Stdio::from_raw_fd(fd_d.1))
                    .stderr(Stdio::from_raw_fd(fd_d.1))
                    .status()
                    .expect("Failed to execute process");
                //eprintln!("status :{:?}\n", output.code().unwrap());
                let exit_code = output.code().unwrap();
                let mut buf: [u8; 1] = [exit_code as u8];
                let lene = write(fd_c.1, &buf).unwrap();
                //println!("{} args {:?}",exit_code,args);

                exit(exit_code);
            }
            close(fd_d.1).unwrap();
            close(fd_c.1).unwrap();
        }
        Err(_) => println!("Fork failed"),
    }
}
