use std::fmt::{self, Debug, Display};

/// A 2D grid for simple terminal graphics
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct View {
    pub width: u16,
    pub height: u16,
    pub symbol_off: char,
    pub symbol_on: char,
    // @TODO: optimize to use a Vec<usize> with tons of bit-sorcery
    buffer: Vec<bool>,
}

impl View {
    pub fn new(width: u16, height: u16) -> Self {
        let buf_size = (width * height) as usize;
        let buffer = std::iter::repeat(false).take(buf_size).collect();

        Self {
            width,
            height,
            buffer,
            symbol_off: '░',
            symbol_on: '█',
        }
    }

    /// Set the characters to print for a coordinate's value.
    ///
    /// Defaults are `off` = `░`, `on` = `█`
    pub fn set_symbols(mut self, off: char, on: char) -> Self {
        self.symbol_off = off;
        self.symbol_on = on;
        self
    }

    /// Dimensions as a (width, height) tuple
    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

impl View {
    #[inline(always)]
    fn index(&self, x: u16, y: u16) -> usize {
        (y * self.height + x) as usize
    }

    #[inline(always)]
    fn bounds_check(&self, x: u16, y: u16) -> bool {
        return x < self.width && y < self.height;
    }

    fn checked_index(&self, x: u16, y: u16) -> Option<usize> {
        self.bounds_check(x, y).then(|| self.index(x, y))
    }
}

impl View {
    /// Returns value at given coordinate while checking if valid.
    pub fn at(&self, x: u16, y: u16) -> Option<bool> {
        self.checked_index(x, y)
            .map(|idx| self.buffer[idx as usize])
    }

    /// Returns value at given coordinate panicking if outside view.
    ///
    /// Panics if coordinate is outside View
    #[inline(always)]
    pub fn unchecked_at(&self, x: u16, y: u16) -> bool {
        self.at(x, y).expect("Given coordinate not inside View")
    }

    ///// Swaps the values at two coordinates
    //pub fn swap(&mut, x1: u16, y1: u16, x2: u16, y2: u16) {}

    /// Set the value at a coordinate
    ///
    /// Returns whether it was successful or not, i.e. the coordinate was within bounds
    pub fn set(&mut self, x: u16, y: u16, value: bool) -> bool {
        if let Some(idx) = self.checked_index(x, y) {
            self.buffer[idx] = value;
            true
        } else {
            false
        }
    }

    /// Iterate over View with coordinates
    ///
    /// Item resembles `(x, y, value)`
    pub fn iter<'v>(&'v self) -> ViewIter<'v> {
        ViewIter {
            view: self,
            x: 0,
            y: 0,
        }
    }
}

impl Display for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf_size = self.height * (self.width + 1); // +1 to include newlines
        let mut buf = String::with_capacity(buf_size as usize);

        self.iter().for_each(|(x, _, val)| {
            buf.push(if val { self.symbol_on } else { self.symbol_off });

            if x == self.width - 1 {
                buf.push('\n');
            }
        });

        write!(f, "{buf}")
    }
}

/// Iterate over a complete View
pub struct ViewIter<'v> {
    view: &'v View,
    x: u16,
    y: u16,
}

impl Iterator for ViewIter<'_> {
    // (x, y, val)
    type Item = (u16, u16, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self
            .view
            .checked_index(self.x, self.y) // uses checked so it return none when done
            .and_then(|idx| self.view.buffer.get(idx))
            .map(|val| (self.x, self.y, *val));

        self.x += 1;
        if self.x == self.view.width {
            self.x = 0;
            self.y += 1;
        }

        val
    }
}

// @TODO: write tests
#[test]
fn at() {
    unimplemented!()
}

#[test]
fn unchecked_at() {
    unimplemented!()
}

#[test]
#[should_panic]
fn unchecked_at_panic() {}
