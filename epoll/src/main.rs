fn main() {
    println!("Hello, world!");
}


struct wait_queue_head_t<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {
    fn new() -> Self {
        wait_queue_head_t{ qdata: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.qdata.push(item);
    }

    fn pop(&mut self) ->Option<T> {
        let l = self.qdata.len();

        if l > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }
}

struct list_head {

}

struct rb_root {

}

struct eventpoll {
    wq: wait_queue_head_t,
    rdllist: list_head,
    rdr: rb_root,

}