# Jole's Dither implementation

jd-rs will dither images according to a palette (or the default one). It will
read fardbeld images from stdin, and then write the result to stdout. See the
test for an example.

![before](tests/img/lain.jpg)
![after](tests/img/lain_ans.png)

## Usage
`jd` reads a fardbeld image on stdin and outputs a corresponding dithered farbfeld image on stdout.
```sh
2ff <cute.png | cargo run -r | ff2png >cuter.png
```

You can also pass [a palette](https://lospec.com/palette-list/twilight-5)
```sh
echo -e 'fbbbad\nee8695\n4a7a96\n333f58\n292831' > pal
2ff <cute.png | cargo run -r -- pal | ff2png >cutest.png
```

## Todo
* palette generation
* threaded video dithering ([jdv](https://github.com/joleeee/jd/blob/master/jdv)]
