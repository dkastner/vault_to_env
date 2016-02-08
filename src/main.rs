extern crate hyper;
extern crate libc;
extern crate regex;

use libc::execv;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use hyper::Client;
use hyper::header::Connection;

struct Secret {
  env_var: String,
  vault_path: String,
  vault_key: String
}

impl Secret {

  fn value(&self) -> Result<String, String> {

    /*let result = Vault.logical.read(vault_path)*/

    /*if val.nil?*/
      /*STDERR.puts("Secretfile: Vault doesn't know about #{vault_path}")*/
      /*exit(1)*/
      /*end*/
    /*if secret.data[vault_key].nil?*/
      /*avail = secret.data.keys.join(', ')*/
      /*STDERR.puts(<<"EOD")*/
  /*Secretfile: Asked for #{vault_path}:#{vault_key}, but only have: #{avail}*/
  /*EOD*/
      /*exit(1)*/
    /*end*/
    /*if secret.lease_duration < 2592000*/
      /*STDERR.puts(<<"EOD")*/
  /*ERROR: #{vault_path} will expire in less than 30 days*/
  /*Can't use vault_to_env safely.  Please extend expiration or use vault gem.*/
  /*EOD*/
      /*exit(1)*/
    /*end*/

    return Ok("VAL".to_string());

  }

}

fn main() {
  let secrets = load_secrets().unwrap();

  for secret in secrets {
    env::set_var(secret.env_var, try!(secret.value()));
  }

  // Execute the wrapped program.
  let args: std::env::Args = std::env::args();
  let arg_list: Vec<_> = args.collect();

  if args.len() > 1 {
    let prog: &[u8] = try!(args.nth(1));
    let exec_args: Vec<_> = arg_list.split_first().last();

    for a in exec_args {
      exec_args.push(a);
    }

    libc::execv(prog, exec_args);
  }
}

fn load_secrets() -> Result<Vec<Secret>, String> {
  let file = match File::open("Secretfile") {
    Ok(f) => f,
    Err(e) => {
      println!("Unable to open Secretfile: {}", e);
      return Err(e.to_string());
    }
  };
  let mut reader = BufReader::new(file);

  let mut secrets = Vec::new();
  let secret_line = Regex::new(r"\A([^ ]+) ([^:]+):(.+)\z").unwrap();
  let comment_line = Regex::new(r"\A#").unwrap();

  for line in reader.lines() {
    let l: String = try!(line);

    if comment_line.is_match(&l) {
      // next
    } else if secret_line.is_match(&l) {
      let caps: regex::Captures = secret_line.captures(&l).unwrap();
      secrets.push(Secret {
        env_var: caps.at(1).unwrap().to_string(),
        vault_path: caps.at(2).unwrap().to_string(),
        vault_key: caps.at(3).unwrap().to_string()
      });
    } else {
      return Err(format!("Can't parse line {}", l));
    }

  }

  return Ok(secrets);
}
