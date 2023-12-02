# Momou

*Javascript is hard to trace. I tried to help it out. I failed.*
It is a pain to write bigger code in plain JS, since the callbacks and function will become a mess, without much structure.
<br>
I thought a program that could identify individual functions and their invocations would help with program tracing.
<br>
I originally wanted more features like side-effect detection, but ran of motivation before then.
<br>
It kinda looks pretty tho.

<hr>

### Usage: 
*Annotations:* To help `Momou` identify the class of an object, annotate the variable with the following syntax:
```js
class Class {
    constructor () {}
    func () {}
}
// @class Class
const object = new Class ();
object.func();
```
build & host the webpage: `source host.sh`
<br>
build the graph for your javascript file: `source rebuild.sh tests/test.js`

<hr>

### Limitations:
- Cannot trace all invocations.
- Cannot trace class heritage (v-tables)

<hr>

### Technical Notes:
The backend parser queries `Treesitter`, and the frontend is built with `d3.js`.
