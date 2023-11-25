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

function a () {
  console.log("hello world");
  let a = 0;
  a += 1;
}

let state = 0;
function b () {
  console.log("mutating!");
  state += 1;
}

function invokationTester () {
	b()
	a()
	a()()
}

class A {
  constructor () {}
  a () {}
}
class B extends A{
  constructor () {
    super()
  }
  b () {}
}

function annotationTester () {
	//@class A
	const hello = new A();
	
	//@class B
	let notHello = new B();
	
	hello.a()
	notHello.b()

  basic()
}

const basic = () => {}
const embedded = () => {
  const f = () => {

  }
  setTimeout(() => f(), 100);
  setTimeout(function () { f() }, 100);
}

const obj = {
  a: () => {},
  b: function () {}
}
