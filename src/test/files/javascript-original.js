// @ts-nocheck

// Basic JavaScript with most keywords

var x = 10; // declaration with var
let y = 20; // declaration with let
const z = 30; // declaration with const

function exampleFunction(a, b) {
  try {
    // try block
    if (a === 0) {
      throw "Division by zero"; // throw statement
    }
    let result = a / b; // local variable
    return result; // return statement
  } catch (error) {
    // catch block
    console.error(error); // console logging
  } finally {
    // finally block
    console.log("Operation completed");
  }
}

class ExampleClass {
  constructor() {
    this.property = 100; // object property
  }

  method() {
    // method of the class
    return "Hello, World!";
  }

  static staticMethod() {
    // static method
    return "Static method called";
  }
}

let obj = new ExampleClass(); // object creation
console.log(obj.method()); // calling method
console.log(ExampleClass.staticMethod()); // calling static method

if (x > y) {
  // if statement
  console.log("x is greater than y");
} else if (x === z) {
  // else if statement
  console.log("x is equal to z");
} else {
  // else statement
  console.log("x is less than y and z");
}

for (let i = 0; i < 5; i++) {
  // for loop
  console.log(i);
}

while (y > 0) {
  // while loop
  y--;
}

do {
  // do-while loop
  console.log("Do-while loop executed");
} while (false);

switch (x) {
  case 10:
    console.log("x is 10");
    break; // break statement
  case 20:
    console.log("x is 20");
    break;
  default:
    console.log("Default case");
}

function* generatorExample() {
  yield 1; // generator yield
  yield 2;
  return 3; // generator return
}

let gen = generatorExample();
console.log(gen.next().value); // using generator
console.log(gen.next().value);
console.log(gen.next().value);

const obj2 = {
  prop1: "value1",
  prop2: "value2",
  [Symbol('key')]: "symbolKey", // computed property
};

for (let key in obj2) {
  // for-in loop
  console.log(key + ": " + obj2[key]);
}

const numbers = [1, 2, 3, 4];
for (const num of numbers) {
  // for-of loop
  console.log(num);
}

function* asyncGenerator() {
  yield new Promise((resolve) => resolve("async yield"));
}

async function asyncFunction() {
  // async function
  let result = await asyncGenerator().next();
  console.log(result.value);
}

asyncFunction();

let condition = true;
do {
  break; // break used in loop
} while (condition);

continue; // continue statement (not in loop here, just example)
delete obj2.prop1; // delete operator

// Try out with blocks as well
{
  let a = 50;
  const b = 60;
  console.log(a, b);
}

export default exampleFunction; // export
import exampleFunction from './example.js'; // import

eval("console.log('Evaluated code');"); // eval (unsafe)
with (Math) {
  console.log(sqrt(16)); // with statement
}
