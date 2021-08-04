//! TableStream gives you a tool for streaming tables out to the terminal.
//! It will buffer some number of rows before first output and try to automatically
//! determine appropriate widths for each column.

use std::{
    cmp::max,
    fmt::{self, Write as FmtWrite},
    io::{self, Write},
    marker::PhantomData,
    mem
};

#[cfg(test)]
mod tests;

pub struct Stream<T, Out: Write> {
    columns: Vec<Column<T>>,
    width: usize, // calculated.
    max_width: usize,
    grow: bool,
    output: Out,

    #[allow(dead_code)] // TODO
    wrap: bool,
    borders: bool,
    padding: bool,

    sizes_calculated: bool,
    buffer: Vec<T>,

    // It's handy to have a long-lived string buffer so we don't have to continue to reallocate.
    str_buf: String,

    _pd: PhantomData<T>,
}


impl <T, Out: Write> Stream<T, Out> {

    /// Create a new table streamer.
    pub fn new(output: Out, columns: Vec<Column<T>>) -> Self {
        Self{
            columns,
            max_width: 0,
            width: 0, // calculated later.
            grow: true,
            output,
            wrap: false,
            borders: false,
            padding: true,

            sizes_calculated: false,
            buffer: vec![],

            str_buf: String::new(),

            _pd: Default::default(),
        }.max_width(
            crossterm::terminal::size().map(|(w,_)| w as usize).unwrap_or(80)
        )
    }



    pub fn borders(mut self, borders: bool) -> Self {
        self.borders = borders;
        let width = self.max_width;
        self.max_width(width)
    }

    /// Set the maximum width for the table.
    /// Note: this may be increased automatically for you if you've
    /// specified columns, borders, dividers, and paddings with sizes
    /// that require a larger max_width.
    pub fn max_width(mut self, max_width: usize) -> Self {
        let num_cols = self.columns.len();
        let padding = if self.padding { 1 } else { 0 };
        let dividers = (num_cols - 1) * (1 + 2*padding);
        let border = if self.borders { 1 } else { 0 };
        let borders = border * (border + padding) * 2;

        let col_widths = self.columns.iter().map(|c| c.min_width).sum::<usize>();
        let min_width = col_widths + borders + dividers;
        self.max_width = max(max_width, min_width);
        self
    }

    /// Print a single row.
    /// Note: Stream may buffer some rows before it begins output to calculate 
    /// column sizes.
    pub fn row(&mut self, data: T) -> io::Result<()> {

        if self.sizes_calculated {
            return self.print_row(data);
        }
        
        self.buffer.push(data); 
        if self.buffer.len() > 100 {
            self.write_buffer()?;
        }
        
        Ok(())
    }

    fn write_buffer(&mut self) -> io::Result<()> {
        self.calc_sizes()?;

        self.print_headers()?;

        let buffer = mem::replace(&mut self.buffer, vec![]);
        for row in buffer {
            self.print_row(row)?;
        }

        Ok(())
    }

    fn print_headers(&mut self) -> io::Result<()> {
        self.hr()?;
        if self.borders {
            write!(&mut self.output, "|")?;
            if self.padding {
                write!(&mut self.output, " ")?;
            }
        }

        let divider = if self.padding {
            " | "
        } else {
            "|"
        };

        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                write!(&mut self.output, "{}", divider)?;
            }
            let mut name = col.header.as_ref().map(|h| h.as_str()).unwrap_or("").to_string();
            safe_truncate(&mut name, col.width);
            write!(&mut self.output, "{:1$}", name, col.width)?;
        }

        if self.borders {
            if self.padding {
                write!(&mut self.output, " ")?;
            }
            write!(&mut self.output, "|")?;
        }

        writeln!(&mut self.output, "")?;
        
        self.hr()?;

        Ok(())
    }

    fn hr(&mut self) -> io::Result<()> {
        writeln!(&mut self.output, "{1:-<0$}", self.width, "")
    }

    fn print_row(&mut self, row: T) -> io::Result<()> {

        let buf = &mut self.str_buf;
        let out = &mut self.output;

        if self.borders {
            write!(out, "|")?;
            if self.padding {
                write!(out, " ")?;
            }
        }

        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                if self.padding {
                    write!(out, " | ")?;
                } else {
                    write!(out, "|")?;
                }
            }

            buf.clear();
            write!(buf, "{}", (col.mapper)(&row)).to_io()?;

            // TODO: This assumes characters are all 1 wide.
            // Crate to calculate glyph/column widths?
            if buf.len() > col.width {
                safe_truncate(buf, col.width);
            }

            write!(out, "{0:1$}", buf, col.width)?;

        }

        if self.borders {
            if self.padding {
                write!(out, " ")?;
            }
            write!(out, "|")?;
        }

        writeln!(out, "")?;

        Ok(())
    }

    fn calc_sizes(&mut self) -> io::Result<()> {
        if self.sizes_calculated { return Ok(()); }
        self.sizes_calculated = true; // or will be very soon. :p


        for row in &self.buffer {
            for col in self.columns.iter_mut() {
                self.str_buf.clear();
                write!(&mut self.str_buf, "{}", (col.mapper)(row)).to_io()?;
                col.max_width = max(col.max_width, self.str_buf.len());
                col.width_sum += self.str_buf.len();
            }
        }

        let num_cols = self.columns.len();
        let padding = if self.padding { 1 } else { 0 };
        let dividers = (num_cols - 1) * (1 + 2*padding);
        let border = if self.borders { 1 } else { 0 };
        let borders = border * (border + padding) * 2;
        let available_width = self.max_width - borders - dividers;


        // First attempt:
        // Simple calculation: Just give every column its max width.
        let col_width = |c: &Column<T>| { 
            let mut width = max(
                c.max_width,
                c.header.as_ref().map(|h| h.len()).unwrap_or(0)
            );
            width = max(width, c.min_width);
            width
        };

        let all_max: usize = self.columns.iter().map(col_width).sum();
        if all_max < available_width {
            // easy mode, just give everyone their max.
            for col in self.columns.iter_mut() {
                col.width = col_width(col);
            }
            self.width = all_max + dividers + borders;

            if self.grow {
                // TODO: Grow to fill available width, even though unnecessary.
                // TODO: Always grow?
            }
            return Ok(());
        }

        // What we have doesn't fit in the given width.
        // BAD IDEA: Allocate cols according to how much we're trying to shove into them.
        // This fails because one verbose column eats all the width, "penalizing" other columns
        // that are displaying nice terse data.
        // INSTEAD: "penalize" the verbose columns, by giving them the remainder after
        // allowing the less verbose columns to use their max_width.

        for big_cols in 1..=self.columns.len() {
            // We expect that when verbose_cols=self.columns.len(), we'll just divide 
            // the available columns among the columns. This should only fail in
            // pathological cases where there are just too many cols to display period.
            // TODO: Just increase Stream.max_width if the user has passed us something broken. End user can resize window.
            if self.penalize_big_cols(big_cols) {
                self.width = self.max_width;
                return Ok(())
            }
        }

        panic!("Couldn't display {} columns worth of data in {} columns of text", self.columns.len(), self.max_width);
    }

    fn penalize_big_cols(&mut self, num_big_cols: usize) -> bool {
        let num_cols = self.columns.len();
        let padding = if self.padding { 1 } else { 0 };
        let dividers = (num_cols - 1) * (1 + 2*padding);
        let border = if self.borders { 1 } else { 0 };
        let borders = border * (border + padding) * 2;
        let available_width = self.max_width - borders - dividers;

        let mut col_refs: Vec<_> = self.columns.iter_mut().collect();
        col_refs.sort_by_key(|c| c.width_sum); // sort "big" cols to the end:
        let (small_cols, big_cols) = col_refs.split_at_mut(num_cols - num_big_cols);

        let needed_width: usize = 
            small_cols.iter().map(|c| max(c.min_width, c.max_width)).sum::<usize>()
            + big_cols.iter().map(|c| c.min_width).sum::<usize>();

        if needed_width > available_width {
            return false
        }

        // Small cols all get their max width. Yay!
        let mut remaining_width = available_width;
        for col in small_cols.iter_mut() {
            col.width = max(col.min_width, col.max_width);
            remaining_width -= col.width;
        }

        // Big cols get assigned the remaining sizes.
        // First pass, try assigning widths w/ simple algorithm.  If the column has a min_width that is
        // larger, subtract the width from available cols, which we'll reallocate on the 2nd pass.
    
        let mut big_cols_left = num_big_cols;
        for col in big_cols.iter_mut() {
            let alloc_width = ((col.width_sum as f64 / num_big_cols as f64) * remaining_width as f64).floor() as usize;
            if alloc_width < col.min_width {
                col.width = col.min_width;
                remaining_width -= col.width;
                big_cols_left -= 1;
            }
        }

        // Second pass: allocate remaining cols:
        if big_cols_left > 0 {
            let cols_per_big_col = remaining_width / big_cols_left;
            for col in big_cols.iter_mut() {
                if col.width > 0 { continue; } // already calculated.
                col.width = cols_per_big_col;
            }   
            
            remaining_width -= big_cols_left * cols_per_big_col;

            // If we have any left, put it in the biggest column:
            if remaining_width > 0 {
                for col in big_cols.iter_mut().rev().take(1) {
                    col.width += remaining_width;
                }
            }
        }

        true
    }

    /// Finish writing output.
    /// This may write any items still in the buffer,
    /// as well as a trailing horizontal line.
    pub fn finish(mut self) -> io::Result<()> {
        self.write_buffer()?;
        self.hr()?;

        Ok(())
    }
}

/// Configure how we want to display a single column.
pub struct Column<T> {
    header: Option<String>,
    mapper: Box<dyn Fn(&T) -> &dyn fmt::Display>,

    // TODO: alignment.

    // Min size specified by user
    min_width: usize,


    // size calculated by algorithm.
    width: usize,

    // Temp vars used while calculating the width:

    max_width: usize, // max size encountered in buffer data.
    width_sum: usize, // sum of widths of all rows. Used to weigh column widths.

    _pd: PhantomData<T>,
}

impl <T> Column<T> {

    /// Create a new Column with a Fn that knows how to extract one column of data from T.
    pub fn new<F>(func: F) -> Self 
    where F: (Fn(&T) -> &dyn fmt::Display) + 'static
    {
        Self {
            header: None,
            mapper: Box::new(func),

            // a min-width of 1 means we'll always at least show there was *some* data in a col,
            // even if it's truncated.
            min_width: 1,
            width: 0,
            max_width: 0,
            width_sum: 0,


            _pd: Default::default(),
        }
    }

    pub fn header(mut self, name: &str) -> Self {
        self.header = Some(name.to_string());
        self.min_width = max(self.min_width, name.len());
        self
    }

    pub fn min_width(mut self, min_width: usize) -> Self {
        self.min_width = min_width;
        self
    }
}

fn safe_truncate(value: &mut String, mut len: usize) {
    if value.len() <= len { return }

    // truncate panics if you try to truncate at a non-char-boundary. >.< 
    while !value.is_char_boundary(len) {
        len -= 1;
    }

    value.truncate(len);
}


trait ToIOResult {
    fn to_io(self) -> io::Result<()>;
}

impl ToIOResult for fmt::Result {
    fn to_io(self) -> io::Result<()> {
        self.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}