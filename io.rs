// Rust IO struct for Baekjoon Online Judge
// https://github.com/POMMI3R/RustIO

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

    fn line(&mut self) -> String {
        let mut s = String::new();
        self.input.read_line(&mut s).unwrap();
        s
    }

    fn all_line(&mut self) -> Vec< String > {
        let mut s = String::new();
        self.input.read_to_string(&mut s).unwrap();
        s.split('\n').map(|s| String::from(s)).collect()
    }

    fn l2v< U: std::str::FromStr >(&mut self) -> Vec< U >
    where < U as std::str::FromStr >::Err: std::fmt::Debug {
        let mut s = String::new();
        self.input.read_line(&mut s).unwrap();
        s.split_whitespace().map(|s| s.parse().unwrap()).collect()
    }

    fn a2v< U: std::str::FromStr >(&mut self) -> Vec< U >
    where < U as std::str::FromStr >::Err: std::fmt::Debug {
        let mut s = String::new();
        self.input.read_to_string(&mut s).unwrap();
        s.split_whitespace().map(|s| s.parse().unwrap()).collect()
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
