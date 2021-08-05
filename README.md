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

 * Doesn't handle right-to-left text. (Have tips on doing this in a terminal!?)
 * Emoji aren't handled.
 * Bengali seems to not render properly in Widnows terminal, so that's not supported.
   (Though maybe it'll work for you elsewhere?)

Future Features?
----------------

 * Could add options for colors.

