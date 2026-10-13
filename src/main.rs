use sha2::{Sha256, Digest};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender};
use clap::Parser;

/// hngen (Head Nonce Generator) is a CLI tool to find a nonce
/// which hashes to a value beginning with a specified character string.
#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
struct Args {
  /// String to search for
  #[arg(short = 's', long)]
  head: String,
  /// Number of threads to dispatch
  #[arg(short, long, default_value_t = 16, value_parser = notzero)]
  threads: u8,
  /// Instruct threads to print all hashes
  #[arg(long)]
  verbose: bool
}

fn notzero(s: &str) -> Result<u8, String> {
  let n = s.parse::<u8>();
  match n {
  Ok(n) => {
    if n != 0 {
        Ok(n)
    } else {
        Err(String::from("Must dispatch at least one thread"))
    }
  }
  Err(_) => {Err(String::from("Must enter a valid 8-bit number (1-255)"))}
  }
}

struct Nonce {
  n: usize,
  h: String
}

fn threadgen(id: u8, tx: Sender<Nonce>, args: Args) {
  let mut h: String;
  let mut n: String = String::new();
  let s = args.head.as_str();

  for _ in 0..id {
    n += "0";
  }

  let mut a1: String = String::new();
  for _ in 0..args.threads {
      a1 += "0";
  }
  let a: &str = a1.as_str();

  loop {
    let nstr = n.to_owned();
    h = format!("{}", hex::encode(Sha256::digest(nstr)));

    if args.verbose {
      println!("{} from #{}: {}", n.len(), id, h);
      // Alternate debug:
      // println!("{}", n.len());
    }

    if h.starts_with(s) {
      tx.send(Nonce { n: n.len(), h }).unwrap();
      break;
    }

    n += a;
  }
}

fn main() {
  let args = Args::parse();
  let (tx, rx) = mpsc::channel();
  let mut handles:Vec<JoinHandle<()>> = Vec::new();

  println!("Searching for nonce hashing to {}...", args.head);

  for thread in 0..args.threads {
      let txn = tx.clone();
      let argsn = args.clone();
      handles.push(thread::spawn(move || {
        threadgen(thread, txn, argsn);
      }));
  }

  let rec = rx.recv().unwrap();

  for handle in handles {
    // We should be able to ignore errors caused by early thread termination
    handle.join().unwrap_or(())
  }

  if args.verbose {println!("")}

  println!("Nonce of length {} hashes to:\n{}", rec.n, rec.h);
}
