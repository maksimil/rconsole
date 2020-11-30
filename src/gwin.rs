use crate::term;

#[derive(Debug, Clone)]
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
        self.buff[j].replace_range(i..i + s.len(), s)
    }

    pub fn buffer(&self) -> &Vec<String> {
        &self.buff
    }
}

pub struct Window {
    buff: GBuffer,
}

impl Window {
    pub fn new(size: (usize, usize)) -> Window {
        term::clear();
        Window {
            buff: GBuffer::new(size),
        }
    }

    pub fn push_buffer(&mut self, buff: GBuffer) {
        if self.buff.size() != buff.size() {
            panic!("Buffer sizes don't match");
        }

        let (width, _) = self.buff.size();

        for (i, (achars, aline, bchars)) in buff
            .buffer()
            .iter()
            .zip(self.buff.buffer().iter())
            .map(|(a, b)| {
                (
                    a.split("").collect::<Vec<_>>(),
                    a,
                    b.split("").collect::<Vec<_>>(),
                )
            })
            .enumerate()
        {
            let mut j = 0;
            while j < width {
                if achars[j] != bchars[j] {
                    let mut k = j + 1;
                    while k < width && achars[k] != bchars[k] {
                        k += 1;
                    }

                    term::moveto(((j - 1) as u16, i as u16));
                    term::putline(&aline[j - 1..k - 1]);

                    j = k - 1;
                }

                j += 1;
            }
        }

        self.buff = buff;
    }

    pub fn edit_buffer<F: FnOnce(GBuffer) -> GBuffer>(&mut self, edit: F) {
        self.push_buffer(edit(self.buff.clone()));
    }
}
