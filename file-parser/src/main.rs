// Problem : i need the detailed information of ports
//           this can be obtained from a file /usr/share/nmap/nmap-services
//           here this code will try to parse it and create a rust hash map to stdout
//           it should look something like this :
//                  m.insert(80,  "http        tcp, udp, sctp");
//                  m.insert(443, "https       tcp, udp, sctp");

use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
    };

fn main() {
    let mut out_writer = Box::new(File::create("output").unwrap()) as Box<dyn Write>;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./port-details") {
        let mut current_port: u16 = 0 ;
        for line in lines {
            if let Ok(detail) = line {
                let mut results = detail.split_whitespace();
                let service = results.next().unwrap();
                let port_proto = results.next().unwrap();
                let mut port_proto = port_proto.split('/');
                let port = port_proto.next().unwrap().parse::<u16>().unwrap();
                if port == current_port {
                    continue;
                }
                current_port = port;
                let proto = port_proto.next().unwrap();
                out_writer
                    .write(format!("m.insert({}, \"{}       {}\");\n", port, service, proto).as_bytes())
                    .ok();
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}