extern crate nix;

use std::os::unix::process::ExitStatusExt;
use nix::sys::wait::waitpid;
use nix::unistd::{alarm, close, fork, pipe, read, ForkResult, write};
use std::os::unix::io::FromRawFd;
use std::process::{Command, Stdio};
use std::{format,env,str,fmt};

use crate::config::{ProgConfig, SeedConfig}; 

pub fn exec_fuzz(seed_config: &SeedConfig, prog_config: &ProgConfig) {
    let fd_d = pipe().unwrap();
    let fd_c = pipe().unwrap();
    //let args: Vec<String> = env::args().collect();
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            eprintln!("Inside parent ,child pid is {}", child);
            let mut arr: [u8; 1] = [0; 1];
            close(fd_d.1);
            close(fd_c.1);

            /**TODO
              Implement timeout
              **/

      
            let mut len = [1,1];
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

            if control[0] != 0 {


            } 

            println!("Data {}\nControl {:?}", String::from_utf8(data).unwrap(),control[0]);

        }
        

        Ok(ForkResult::Child) => {
            close(fd_d.0);
            close(fd_c.0);
            eprintln!("Inside Child");
            let mut args: Vec<String> = Vec::new();
            args.push(prog_config.inputpath.clone());
            for i in seed_config.seed.clone(){//.into_iter().map(|s| s);
            args.push(i);
            }
          //  println!("{} len {}",seed_config.arg_count,args.len());

            unsafe {
                let output = Command::new(&args[0])
                    .args(&args[1..seed_config.arg_count+1])
                    .stdout()//Stdio::from_raw_fd(fd_d.1))
                    .stderr(Stdio::from_raw_fd(fd_d.1))
                    .status()
                    .expect("Failed to execute process");
                //eprintln!("status :{:?}\n", output.code().unwrap());
                let exit_code = output.code().unwrap();
                let mut buf : [u8;1] = [exit_code as u8];
                let lene = write(fd_c.1,&buf).unwrap();
            }
        }
        Err(_) => println!("Fork failed"),
    }
}
