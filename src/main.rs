use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

struct ProcessInfo {
    pid: u32,
    name: String,
    user: String,
}

fn main() {
    let args = Args::parse();

    println!("Checking conflicts on port {}", args.port);

    let info = get_process_info(args.port);

    match info {
        Some(info) => println!("Process listening on port {} is {}, pid: {}, user: {}",
         args.port, info.name, info.pid, info.user),
        None => println!("No process listening on port {}", args.port),
    }
}
// get_process_info returns the process information for the process listening on the given port
fn get_process_info(port: u16) -> Option<ProcessInfo> {

    #[cfg(target_os = "linux")]
    {
        println!("Running on Linux, TBD");
        // Linux sample output
        // netstat -tulpn | grep :8080
        // tcp        0      0

    }

    #[cfg(target_os = "macos")]
    {
        // println!("Running on MacOS");
        // MacOS sample output
        // lsof -i :8080
        // COMMAND  PID   USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
        // Python  3151 lydell    5u  IPv6 0x75e2d04027e5da91      0t0  TCP *:http-alt (LISTEN)
        let out = std::process::Command::new("lsof")
            .arg("-i")
            .arg(format!(":{}", port))
            .output()
            .expect("failed to execute process");

        if out.status.success() {
            let out_str = std::str::from_utf8(&out.stdout).expect("Could not read lsof output");
            let lines: Vec<&str> = out_str.lines().collect();
            if lines.len() > 1 {
                // This check is to find if more than 1 line was returned - mac os lsof returns 2 lines
                // and the second line is the one we want to parse for information

                // process name is in the first column of the second line
                let process_name = lines[1].split_whitespace().nth(0).unwrap();
                // pid is in the second column of the second line
                let pid = lines[1].split_whitespace().nth(1).unwrap();
                // user is in the third column of the second line
                let user = lines[1].split_whitespace().nth(2).unwrap();
                
                return Some(
                    ProcessInfo { 
                        pid: pid.parse::<u32>().expect("Could not parse pid"), name: process_name.to_string(), user: user.to_string() 
                    });
            }
        }
    }

    None // return None if no process was found
}
