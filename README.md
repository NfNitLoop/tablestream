TableStream
===========

Stream data to the terminal and display it in ASCII tables.

TableStream will buffer some rows and use them to automatically determine appropriate
widths for columns. It will try to automatically detect the current terminal's width
and fit the rows into that width.

See the [API Docs] for examples.

[API Docs]: https://docs.rs/tablestream


Current Limitations
-------------------

 * Currently only really works for ASCII.  
   I'm currently assuming String.len() == column width.
   Is there a good library for calculating/truncating length based on glyph widths?


Future Features?
----------------

 * Could add options for colors.

