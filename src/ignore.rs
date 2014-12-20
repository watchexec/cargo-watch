extern crate regex;

// FIXME: This should use the compile-time regex! macros.
macro_rules! rt_regex(($r:expr) => ({
  match regex::Regex::new($r) {
    Ok(r) => r,
    Err(e) => panic!("Couldn't parse regex: {}", e)
  }
}));

fn filenames() -> Vec<regex::Regex> {
  vec![
    rt_regex!(r"[^.][^r][^s]$"), // FIXME: It should be possible to trigger on non-.rs changes
    rt_regex!(r"^\."),
    rt_regex!(r"~$"),
    rt_regex!(r"^~")
  ]
}

pub fn filename(f: &String) -> bool {
  for fr in filenames().iter() {
    if fr.is_match(f.as_slice()) {
      return true;
    }
  }

  false
}
