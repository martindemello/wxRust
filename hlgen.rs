use std::io;

struct ClassDef {
  parent: Option<~str>,
  name: ~str
}

fn read_lines(fname: ~str) {
  let mut classes = ~[];
  let path = Path(fname);
  let reader = io::file_reader(&path).unwrap();
  while !reader.eof() {
    let line = reader.read_line();
    let l = line.as_slice();
    if (l.starts_with("TClassDef(")) {
      match (l.find('('), l.find(')')) {
          (Some(b), Some(e)) => {
            classes.push(
              ClassDef { name: l.slice(b + 1, e).to_owned(), parent: None })
          }
          _ => { }
      }
    }
    if (l.starts_with("TClassDefExtend(")) {
      match (l.find('('), l.find(','), l.find(')')) {
          (Some(b), Some(m), Some(e)) => {
            classes.push(
              ClassDef {
                name: l.slice(b + 1, m).to_owned(),
                parent: Some(l.slice(m+1, e).to_owned())
              })
          }
          _ => { }
      }
    }
  }

  for c in classes.iter() {
    match c.parent {
      None => { io::println(c.name) }
      Some(ref p) => { io::println(fmt!("%s : %s", c.name, *p)) }
    }
  }
}

#[main]
fn main() {
  read_lines(~"wxHaskell/wxc/src/include/wxc.h");
}
