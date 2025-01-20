// @ts-nocheck

// TypeScript with most keywords and features

let x: number = 10; // number type
let y: number = 20; // number type
const z: number = 30; // constant with number type

function exampleFunction(a: number, b: number): number {
  try {
    // try block
    if (a === 0) {
      throw "Division by zero"; // throw statement
    }
    let result: number = a / b; // local variable with type
    return result; // return statement
  } catch (error: any) {
    // catch block with any type for error
    console.error(error); // console logging
  } finally {
    // finally block
    console.log("Operation completed");
  }
}

class ExampleClass {
  property: number; // type of property

  constructor() {
    this.property = 100; // object property with type
  }

  method(): string {
    // method with return type
    return "Hello, World!";
  }

  static staticMethod(): string {
    // static method with return type
    return "Static method called";
  }
}

let obj: ExampleClass = new ExampleClass(); // object creation
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

for (let i: number = 0; i < 5; i++) {
  // for loop with type annotation
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

function* generatorExample(): Generator<number, number, unknown> {
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

const numbers: number[] = [1, 2, 3, 4]; // array with type annotation
for (const num of numbers) {
  // for-of loop
  console.log(num);
}

async function asyncFunction(): Promise<void> {
  // async function with return type
  let result = await asyncGenerator().next();
  console.log(result.value);
}

async function* asyncGenerator(): AsyncGenerator<string, string, unknown> {
  yield new Promise<string>((resolve) => resolve("async yield"));
}

asyncFunction();

let condition: boolean = true;
do {
  break; // break used in loop
} while (condition);

delete obj2.prop1; // delete operator

// Try out with blocks as well
{
  let a: number = 50;
  const b: number = 60;
  console.log(a, b);
}

type ExampleType = {
  id: number;
  name: string;
};

const exampleObject: ExampleType = {
  id: 1,
  name: "TypeScript Object",
};

interface Person {
  name: string;
  age: number;
}

const person: Person = {
  name: "John",
  age: 25,
};

enum Status {
  Active = "ACTIVE",
  Inactive = "INACTIVE",
  Pending = "PENDING",
}

const currentStatus: Status = Status.Active;

type Callback = (error: Error | null, result?: string) => void;

const callbackExample: Callback = (error, result) => {
  if (error) {
    console.error(error);
  } else {
    console.log(result);
  }
};

// Generics
function identity<T>(value: T): T {
  return value;
}

console.log(identity(42)); // number type
console.log(identity("hello")); // string type

// Tuple with types
let tuple: [string, number] = ["hello", 42];

export default exampleFunction; // export
import { exampleFunction } from './example'; // import

eval("console.log('Evaluated code');"); // eval (unsafe)
with (Math) {
  console.log(sqrt(16)); // with statement (not recommended)
}

declare let globalVar: number; // declare variable (for global types)
