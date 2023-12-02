class Dummy {
  constructor () {}
  member (c,d) {}
  async  asyncMember (j) {}
  static staticMember ({ c, d }) {}
  static async staticAsyncMember () {}

  arrow = ({r, k, l}, y) => {}
  arrowInline = (a) => {};
  asyncArrow = async (a2, l) => {}
  static staticArrow = (js) => {}
  static staticAsyncArrow = async (rust) => {}
}

function lvalueTester () {
  a	 .  	b.  c(	c )	.	 dfunc(d) = 2
  a	 .  	b.  c(	c )	.	 d() = 2
  a	 .  	b.  c(	c )	.	 d() = 2
}

function deterministic () {
  console.log("hello world");
  let a = 0;
  a += 1;
}

let state = 0;
function mutating () {
  console.log("mutating!");
  state += 1;
}

function invoker () {
	deterministic()
	mutating()
}

class Base {
  constructor () {}
  func () {}
}
class Child extends Base {
  constructor () {
    super()
    this.state = 1;
  }
  mutate () {
    this.state ++;
  }
}

function annotationTester () {
	/// @class:Base
	const hello = new Base();
	
	///@class:Child
	let notHello = new Child();
	
	hello.func()
	notHello.mutate()

  emptyArrow()
}

const basic = () => {}
const timeouter_er = () => {
  const embedded = () => {

  }
  setTimeout(() => embedded(), 100);
  setTimeout(function () { embedded() }, 100);
}

const obj = {
  a: () => {},
  b: function () {}
}

/// @class:Base
const hello = new Base();

// :thinking: this isn't a declaration!
/// @class:Child
hello = new Child();

// :imp: a fake!
/// @glass:Child
const fake = new Child();
