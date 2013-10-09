use std::io;
use std::str::CharEq;
use std::cell::Cell;
use std::hashmap::HashMap;

struct ClassDef {
  parent: Option<~str>,
  name: ~str,
  methods: ~[Method]
}

#[deriving(Clone)]
struct Arg {
  name: ~str,
  ctype: ~str
}

#[deriving(Clone)]
struct Method {
  cfn: ~str,
  ret: ~str,
  name: ~str,
  class: ~str,
  args: ~[Arg]
}

struct Function {
  cfn: ~str,
  ret: ~str,
  args: ~[Arg]
}

struct File {
  classes: ~[ClassDef],
  methods: ~[Method],
  functions: ~[Function]
}

enum LineType {
  Include,
  ClassDef,
  ClassDefEx,
  MethodDef,
  FunctionDef,
  Comment,
  Unknown
}

fn find_to<'a, Sep : CharEq>(s: &'a str, p: Sep) -> Option<&'a str> {
  do s.find(p).and_then |e| {
    Some(s.slice(0, e))
  }
}

fn find_between<'a, Sep : CharEq>(s: &'a str, p: Sep, q: Sep) -> Option<&'a str> {
  let cq = Cell::new(q);
  do s.find(p).and_then |b| {
    let t = s.slice_from(b + 1);
    find_to(t, cq.take())
  }
}

fn split_at<'a, Sep : CharEq>(s: &'a str, p: Sep) -> Option<(&'a str, &'a str)> {
  do s.find(p).and_then |e| {
    Some((s.slice_to(e), s.slice_from(e+1)))
  }
}

fn line_type(l: &str) -> LineType {
  if l.starts_with("#include ") {
    Include
  } else if l.starts_with("#") || l.starts_with("/") {
    // Treat preprocessor directives as comments for now
    Comment
  } else if l.starts_with("TClassDef(") {
    ClassDef
  } else if l.starts_with("TClassDefExtend(") {
    ClassDefEx
  } else if l.contains("TSelf(") || l.contains("_Create(") || l.contains("_Ctor(") {
    MethodDef
  } else if l.ends_with(");") {
    // Check for terminal ); as a sanity check
    FunctionDef
  } else {
    Unknown
  }
}

fn to_arg(s: &str) -> Arg {
  match split_at(s.trim(), ' ') {
    Some((n, t)) => { Arg { name: n.to_owned(), ctype: t.to_owned() } }
    _ => Arg { name: ~"EXPAND", ctype: s.to_owned() } // TODO: handle these (see wxc_types)
  }
}

fn read_lines(dir: &str, fname: ~str, out: &mut File) {
  let path = Path(dir).push(fname);
  let maybe_reader = io::file_reader(&path);
  if maybe_reader.is_err() { return }
  let reader = maybe_reader.unwrap();
  while !reader.eof() {
    let line = reader.read_line();
    let l = line.as_slice();
    // We trust the wxc.h source to be machine-generated and regular, so once
    // we identify the line type, we can call unwrap() on the expected pieces
    // without checking for None
    let ltype = line_type(l);
    match ltype {
      Include => {
        // Process include files recursively if they exist in the same directory as wxc.h
        match find_between(l, '"', '"') {
          Some(s) => { read_lines(dir, s.to_owned(), out) }
          _ => { }
        }
      }
      ClassDef => {
        let s = find_between(l, '(', ')').unwrap();
        (*out).classes.push(ClassDef {name: s.trim().to_owned(), parent: None, methods: ~[]});
      }
      ClassDefEx => {
        let s = find_between(l, '(', ',').unwrap();
        let p = find_between(l, ',', ')').unwrap();
        (*out).classes.push(
          ClassDef {
            name: s.trim().to_owned(),
            parent: Some(p.trim().to_owned()),
            methods: ~[]
          })
      }
      MethodDef | FunctionDef => {
        // Clean up slight inconsistencies in the header files
        let l = l.replace(" (", "(");

        // Split the line into its components
        let (ret, l1) = split_at(l, ' ').unwrap();
        let (cfn, l2) = split_at(l1, '(').unwrap();
        let cfn = cfn.trim();
        let l3 = l2.trim().trim_right_chars(& &[')', ';']);
        let args : ~[Arg] = l3.split_iter(',').map(|a| to_arg(a)).collect();

        match ltype {
          MethodDef => {
            // Methods are named wxClassName_MethodName() in the C layer.
            // TODO: We need to get the classname from TSelf instead; there are
            // a few cases where it is different.
            let (class, name) = split_at(cfn, '_').unwrap();
            let meth = Method {
              cfn: cfn.to_owned(),
              ret: ret.to_owned(),
              name: name.to_owned(),
              class: class.to_owned(),
              args: args.clone()
            };
            (*out).methods.push(meth);
          }
          _ => {
            let fun = Function {
              cfn: cfn.to_owned(),
              ret: ret.to_owned(),
              args: args.clone()
            };
            (*out).functions.push(fun);
          }
        }

      }
      _ => { /* Ignore everything else */ }
    }
  }
}

fn fill_classes(file : &mut File) {
  let mut class_map = HashMap::new();

  for c in file.classes.mut_iter() {
    class_map.insert(c.name.clone(), c);
  }

  for m in file.methods.mut_iter() {
    match class_map.find_mut(&m.class) {
      None => { io::println(fmt!("could not find class %s from %s", m.class, m.cfn)); }
      Some(c) => { c.methods.push((*m).clone()) }
    }
  }
}

#[main]
fn main() {
  let mut file = File {classes: ~[], methods: ~[], functions: ~[]};
  read_lines("wxHaskell/wxc/src/include", ~"wxc.h", &mut file);

  fill_classes(&mut file);

  for c in file.classes.iter() {
    io::println(fmt!("class %s {", c.name));
    for m in c.methods.iter() {
      io::println(fmt!("  %s %s", m.ret, m.name));
    }
    io::println("}");
  }
}
