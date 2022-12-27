pub struct Queue {
    val: Vec<char>
}

impl Queue {
    pub fn new() -> Self {
        Queue { val: Vec::new() }
    }

    pub fn add(&mut self, v: char) {
        self.val.push(v);
    }

    pub fn pop(&mut self) {
        self.val.pop();
    }
}