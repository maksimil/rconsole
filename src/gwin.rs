#[derive(Debug)]
pub struct GBuffer {
    buff: Vec<String>,
}

impl GBuffer {
    pub fn new((width, height): (usize, usize)) -> GBuffer {
        let s = (0..width).map(|_| " ").collect::<String>();
        GBuffer {
            buff: (0..height).map(|_| s.clone()).collect::<Vec<String>>(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.buff[0].len(), self.buff.len())
    }

    pub fn putline(&mut self, (i, j): (usize, usize), s: &str) {
        self.buff[i].replace_range(j..j + s.len(), s)
    }
}
