# General Information

<!--toc:start-->

- [General Information](#general-information)
- [TODO](#todo)
<!--toc:end-->

I honestly just wanted to play with some video and audio stuff and while this is a very basic stuff, I am thinking of just writing the wrapper around `yt-dlp` myself, as the current one doesn't do a few of the things I'd like it to do. It will also allow me to have stricter arguments, that will just as well make using the CLI eaier.

# TODO

- [ ] Add progress bar. (This is supported by `yt-dlp` itself. However the output of that command isn't printed by `ytd-rs`, which is why I might just write the wrapper myself.)
- [ ] Maybe add a way to not have the initial directory path hardcoded, but saved as a config somewhere.
- [ ] Support for batch downloads?
