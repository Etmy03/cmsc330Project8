// spellcheck_main.rs: main application 
// TODO: complete the TODO items below

use std::process::exit;         // bare use of exit(num) function to exit program
use std::env::args;             // bare use of args() function to retrieve commandline args
use std::fs;                    // for read_to_string()
use project8::p8_funcs::*;      // use all project8 functions

fn main(){
  let args : Vec<String> = args().collect();  // args() returns an iterator over command-line args
  if args.len() < 4 {                         // collect() them into a vector and
    println!("usage: {} <filename> <dict> <mode>",args[0]);
    exit(1);                                  // o/w exit as no file was given to process
  }
  let fname = &args[1];
  let dict_fname = &args[2];
  let mode = &args[3];

  println!("loading dictionary {dict_fname}");
  let dict = load_string_upper(dict_fname);
  // TODO: load a dictionary file with one of the the functions from p8_funcs
  // ???;

  println!("opening file {fname}");
  let file_text = fs::read_to_string(fname).expect("unable to open file");

  println!("mode: {mode}");

  println!("\nCORRECTED TEXT:");
  match mode.as_str() {
    // TODO: fill in each case by creating an appropriate Corrector then
    // calling correct_string() on file_text with it.  Print the
    // entirety of the corrected text returned by the correct_string()
    // function.

    "mark" => {                 // MarkCorrector
      let mut mc = MarkCorrector::new(">>", "<<");
      let corrected_text = correct_string(&file_text, &dict, &mut mc);
      println!("{}", corrected_text);
    },
    "auto" => {                 // AutoCorrector with show_sub: false
      let mut mc = AutoCorrector::new(&dict, false);
      let corrected_text = correct_string(&file_text, &dict, &mut mc);
      println!("{}", corrected_text);
    }
    "auto_show" => {            // AutoCorrector with show_sub: true
      let mut mc = AutoCorrector::new(&dict, true);
      let corrected_text = correct_string(&file_text, &dict, &mut mc);
      println!("{}", corrected_text);
    }
    _ => {
      println!("Unknown mode");
    }
  }
}
