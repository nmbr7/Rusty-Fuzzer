use libc;
use nix;

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::fuzzstat::FuzzerStatus;
use libc::{c_void, uint8_t};
use libc::{shmat, shmdt, shmget};
use nix::sys::wait::waitpid;
use nix::unistd::{alarm, close, fork, pipe, read, write, ForkResult};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::os::unix::process::ExitStatusExt;
use std::process::{exit, Command, Stdio};
use std::{env, fmt, format, fs, mem, ptr, str};

pub fn exec_fuzz(
    seed_config: &mut SeedConfig,
    prog_config: &ProgConfig,
    fuzz_status: &mut FuzzerStatus,
) {
    let fd_d = pipe().unwrap();
    let fd_c = pipe().unwrap();

    //let args: Vec<String> = env::args().collect();
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            //eprintln!("Inside parent ,child pid is {}", child);
            let mut arr: [u8; 1] = [0; 1];
            close(fd_d.1).unwrap();
            close(fd_c.1).unwrap();
            let mut bitmap: [u8; 4100] = [0; 4100];
            unsafe {
                let shmid = shmget(7015, 4100, 0644 | libc::IPC_CREAT);
                let shmaddr = &shmat(shmid, std::ptr::null_mut(), 0);
                //println!("First seed {}",String::from_utf8_unchecked(seed_config.seed.clone()));
                // println!("shmid {} addr {:?}", shmid, *shmaddr);
                ptr::write_bytes(*shmaddr, 0, 4100);
                let p = *shmaddr as *const u8;
                waitpid(child, None).unwrap();
                for i in 0..4100 {
                    bitmap[i] = *p.add(i);
                    if *p.add(0) > fuzz_status.coverage_count.0 {
                        seed_config.fitness += *p.add(0); // - fuzz_status.coverage_count.1;
                        fuzz_status.coverage_count.0 = *p.add(0);
                        /*println!(
                            "First seed {}\n{:?}",
                            String::from_utf8_unchecked(seed_config.seed.clone()),
                            seed_config.clone()

                        );*/
                        // sleep(Duration::new(2, 0));
                        if *p.add(0) > 4 {
                            seed_config.newlen = seed_config.seed.len();
                        }
                        if *p.add(i) > 0 {
                            //print!("{} ", *p.add(i));
                        }
                    }
                }

                /*println!("\n");
                                for i in 0..4100 {
                                    if *p.add(i) > 0
                                    {
                                        println!("Id {} :{}",i,*p.add(i));
                                    }
                                }
                                println!("\n");
                */
                shmdt(*shmaddr);
            }

            /**TODO
            Implement timeout
            **/
            let mut len = [1, 1];
            let mut data: Vec<u8> = Vec::new();
            let mut control: Vec<u8> = Vec::new();

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
            //println!("First seed {:?}",String::from_utf8(seed_config.seed.clone()));

            /*

            fs::write(
                format!("{}/input_set/{}", prog_config.outputdir, seed_config.input),
                seed_config.seed.as_slice(),
            )
            .unwrap();

            */

            if control[0] != 0 && data.as_slice()[0] == 0x3d {
                seed_config.exit_stat = Stat::CRASH;
                //let c: &[u8] = data.as_slice();
                //let s: &[u8] = seed_config.seed.as_slice();
                fs::write(
                    format!("{}/Crash/{}", prog_config.outputdir, seed_config.inputf),
                    seed_config.seed.as_slice(),
                )
                .unwrap();
                let s = seed_config.seed.clone();

                fs::write(
                    format!("{}/Crash/{}", prog_config.outputdir, seed_config.outputf),
                    data.as_slice(),
                )
                .unwrap();
            }

            //println!("Data {} len {}  Control {}", String::from_utf9(data.clone()).unwrap(),data.len(),control[0]);
            // println!("Fuzzer Status\n {} {} [2J",prog_config);
            close(fd_d.0).unwrap();
            close(fd_c.0).unwrap();
        }

        Ok(ForkResult::Child) => {
            close(fd_d.0).unwrap();
            close(fd_c.0).unwrap();
            // eprintln!("Inside Child");
            let mut args: Vec<&str> = prog_config.prog_args.split(' ').collect();
            //let a : Vec<&str> = Vec::new();
            // a.push(prog_config.prog_name.clone());
            //a.push(seed_config.seed.clone());
            unsafe {
                //args.push(String::from_utf8(seed_config.seed.clone()).unwrap());
                //println!("{:?} len {}", args[1], args.len());
                let output = Command::new(&args[0])
                    .args(&args[1..args.len()])
                    .stdout((Stdio::null()) /*Stdio::from_raw_fd(fd_d.1)*/)
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
    //          println!("{:?}",seed_config);
    //            sleep(Duration::new(2, 0));
}
