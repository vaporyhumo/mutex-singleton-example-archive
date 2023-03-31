use std::sync::Mutex;

struct Record {
  id: usize,
  name: String,
}

struct Num {
  num: usize,
}

impl Num {
  fn add_one(&mut self) -> () { self.num += 1 }
}

struct Repo {
  records: Vec<Record>,
}

impl Repo {
  fn new() -> Self {
    Self { records: vec![Record { id: 1, name: String::from("name") }] }
  }

  fn update(&mut self) -> () {
    self.records.push(Record { id: 1, name: String::from("name") });
  }
}

static REPO: Mutex<Repo> = Mutex::new(Repo { records: vec![] });
static NUM: Mutex<Num> = Mutex::new(Num { num: 0 });

fn main() {
  REPO.lock().unwrap().update();
  REPO.lock().unwrap().update();
  println!("{:?}", foo(1));
  println!("{:?}", foo(2));
}
fn foo(n: usize) -> usize {
  NUM.lock().unwrap().add_one();
  n + 1 + NUM.lock().unwrap().num
}
