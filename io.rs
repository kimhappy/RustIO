use std::io::{ Read, Write, BufReader, BufWriter, BufRead };
use std::fs::File;

struct IO< IT: std::io::Read, OT: std::io::Write > {
    input : BufReader< IT >,
    output: BufWriter< OT >,
    write_buffer: String
}

impl< IT: std::io::Read, OT: std::io::Write > IO< IT, OT > {
    fn new(input: IT, output: OT) -> IO< IT, OT > {
        IO {
            input : BufReader::new( input),
            output: BufWriter::new(output),
            write_buffer: String::new()
        }
    }

    fn with_capacity(input: IT, output: OT, capacity: usize) -> IO< IT, OT > {
        IO {
            input : BufReader::new( input),
            output: BufWriter::new(output),
            write_buffer: String::with_capacity(capacity)
        }
    }

    fn l2v< U: std::str::FromStr >(&mut self) -> Vec< U >
    where < U as std::str::FromStr >::Err: std::fmt::Debug {
        let mut s = String::new();
        self.input.read_line(&mut s).unwrap();
        s.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
    }

    fn a2v< U: std::str::FromStr >(&mut self) -> Vec< U >
    where < U as std::str::FromStr >::Err: std::fmt::Debug {
        let mut s = String::new();
        self.input.read_to_string(&mut s).unwrap();
        s.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
    }

    fn write1< U: std::string::ToString >(&mut self, x: U) {
        self.write_buffer.push_str(&x.to_string());
    }

    fn write2< U: std::string::ToString >(&mut self, x: U, y: char) {
        self.write_buffer.push_str(&x.to_string());
        self.write_buffer.push(y);
    }
}

impl< IT: std::io::Read, OT: std::io::Write > Drop for IO< IT, OT > {
    fn drop(&mut self) {
        self.output.write_all(self.write_buffer.as_bytes()).unwrap();
    }
}

fn main() {
    // Examples
    let mut cio = IO::new(std::io::stdin(), std::io::stdout());
    let mut fio = IO::with_capacity(File::open("input.txt").unwrap(), File::create("output.txt").unwrap(), 128);

    let cv = cio.l2v::< String >();

    for i in cv {
        cio.write1(i);
    }

    let fv = fio.a2v::< i32 >();

    for i in fv {
        fio.write2(i, '\n');
    }
}
