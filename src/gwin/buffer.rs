#[derive(Debug)]
pub struct PutLine_<S: AsRef<str>>(usize, S);

impl<S: AsRef<str>> PutLine_<S> {
    pub fn new(x: usize, s: S) -> Self {
        PutLine_(x, s)
    }

    pub fn sx(&self) -> usize {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.1.as_ref()
    }

    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    pub fn ex(&self) -> usize {
        self.sx() + self.len()
    }
}

type PutLine = PutLine_<String>;
type PutLineStr<'s> = PutLine_<&'s str>;

#[derive(Debug)]
pub struct GBuffer {
    lines: Vec<Vec<PutLine>>,
}

impl GBuffer {
    pub fn new(height: usize) -> GBuffer {
        GBuffer {
            lines: Vec::with_capacity(height),
        }
    }

    pub fn putline_str(&mut self, (i, j): (usize, usize), s: &str) {
        self.putline((i, j), String::from(s));
    }

    pub fn putline(&mut self, (x, y): (usize, usize), s: String) {
        self.expand(y + 1);

        let line = &mut self.lines[y];
        let pl = PutLine::new(x, s);

        if line.len() > 0 && line[line.len() - 1].ex() > pl.sx() {
            panic!("Function putline does not allow overlapping line putting");
        } else {
            line.push(pl);
        }
    }

    pub fn expand(&mut self, height: usize) {
        while self.lines.len() < height {
            self.lines.push(Vec::new());
        }
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    pub fn lines(&self) -> &Vec<Vec<PutLine>> {
        &self.lines
    }
}
