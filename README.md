# Jole's Dither implementation

jd-rs will dither images according to a palette (or the default one). It will
read fardbeld images from stdin, and then write the result to stdout. See the
test for an example.

![before](tests/img/lain.jpg)
![after](tests/img/lain_ans.png)

## Todo
* palette generation
* threaded video dithering ([jdv](https://github.com/joleeee/jd/blob/master/jdv)]