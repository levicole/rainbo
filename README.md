Rainbo
======

It's simple really. Pipe output to the rainbo command, the output now is outputted in full 24 bit color.

This takes advantage of terminals that have [True Color](https://gist.github.com/XVilka/8346728) support. Check the previous link for terminal support.

You may be wondering...Levi, where is the "w"? Well, I simply forgot it when I ran `cargo new` and just ran with it.

## Todo

- [] Pass a file name to command
- [] Should HSV implement the Iterator? That way getting the next color is as simple as saying `hsv.next()`
- [] Should I extract the color lib into it's own project?
- [] How do I distribute this? I don't know, but I will figure it out.
