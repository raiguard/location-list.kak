# location-list.kak

In kakoune, there is no standard for location lists. Many scripts / plugins (including the built-in `grep.kak`) implement a goto kind of functionality, but it is not standardized whatsoever and is easily prone to breakage. `location-list.kak` aims to standardize these lists and provide a common interface for navigating and manipulating location lists.

## What is a location list?

A location list is a list of positions in files. These positions can be jumped to by pressing `enter` on the corresponding line in the location list buffer, or iterated with the various available commands.

Similar to vim, there will be a global location list and client location lists. The global location list spans the entire project, whereas a client list is specific to a client. They are iterated via commands with the `lg` and `lc` prefixes, respectively.

## Internal mechanisms

Each location list is represented by a series of `range-specs` options - one per file in the list. Each `range-specs` has the ranges as usual, and the "arbitrary text" is used for the preview. The first entry in the `range-specs` is a dummy entry used to hold the filename for display.

There is also another `range-specs` that is used for highlighting the contents in a buffer. This one omits the dummy entry and includes the corresponding faces.

Each location list also has an `index` option that corresponds to the current index the user has selected in the list.

### List options

If a list is created in `client0` while in the file `src/main.rs`, the following options are created:

```
lw_client0_index: 0
lw_client0_list_srcmainrs: 0.0+0|src/main.rs 1.1,1.7|kakoune 2.6,2.12|lorem kakoune ipsum
lw_client0_highlights_srcmainrs: 1.1,1.7|LLHighlight 2.6,2.12|LLHighlight
```

If those locations are added to the global list, it will look like this:

```
lg_index: 0
lg_list_srcmainrs: 0.0+0|src/main.rs 1.1,1.7|kakoune 2.6,2.12|lorem kakoune ipsum
lg_highlights_srcmainrs: 1.1,1.7|LLHighlight 2.6,2.12|LLHighlight
```

## The buffer

You can open a location list in a special buffer. Special handling for specific window managers / terminal emulators will need to be added in order to replicate the vim behavior of a horizontal split, so for now it will just open in the current client. You may search and filter this buffer however you wish, but it is read-only in order to preserve line numbers (they correspond with the indices in the list). Pressing enter on a line will jump you to that location in the corresponding client.

For our above example, the buffer will look like this:

```
> src/main.rs:1:1 | kakoune
  src/main.rs:2:6 | lorem kakoune ipsum
```

By default, the preview includes the entire line. There might be options to change this in the future. The `>` in the buffer denotes the currently selected entry.

This buffer updates whenever the list does, so you always have the latest information.

## The commands

There are many commands available for using location lists:

- `(lg|lc)n[ext]`: Go to the next entry on the list.
- `(lg|lc)p[rev]`: Go to the previous entry on the list.
- `(lg|lc)f[irst]`: Go to the first entry on the list.
- `(lg|lc)l[ast]`: Go to the last entry on the list.
- `(lg|lc)o[pen]`: Open the location list.
- `(lg|lc)c[lose]`: Close the location list (TODO: determine how this will work with the splits).
