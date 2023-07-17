// Setting up typescript

// 1. Install Node.js and npm

const { execSync } = require('child_process'); 
execSync('npm install typescript --global');

// 2. Create a tsconfig.json

const tsconfig = {
    "compilerOptions": {
        "target": "es5",
        "module": "commonjs",
        "lib": ["es2015"],
        "sourceMap": true
    },
    "include": [
        "src/**/*"
    ]
};

// 3. Compile your code

const { exec } = require('child_process');
exec('tsc', (error, stdout, stderr) => {
    if (error) {
        console.error(`exec error: ${error}`);
        return;
    }
    console.log(`stdout: ${stdout}`);
    console.log(`stderr: ${stderr}`);
});

// 4. Managing type definitions

const { exec } = require('child_process'); 
exec('tsd install mocha --save', (error, stdout, stderr) => {
    if (error) {
        console.error(`exec error: ${error}`);
        return;
    }
    console.log(`stdout: ${stdout}`);
    console.log(`stderr: ${stderr}`);
});

// 5. Using type aliases

type Alias = { num: number, name: string }
function getName(n: Alias) {
     return n.name;
}

// 6. Generics

function genericFunc<T>(arg: T): T {
    return arg;
}

// 7. Decorators

function logClass(target: any) {
    console.log(`Class: ${target.name}`);
}

@logClass
class Temp { /* ... */ }

// 8. Using namespaces

namespace Tools {
    export function success(message: string) {
        console.log(`Success: ${message}`);
    }
}

// 9. Using external modules

import * as config from './config';

// 10. Writing custom typings

declare module 'tweet' {
    export interface Tweets {
        user: string;
        message: string;
        date: Date;
    }
}

// 11. Performing type assertions

let str = 'Hello, world!';
let myStr: string = <string>str;

// 12. Utilizing type guards

function isNumber(x: any): x is number {
    return typeof x === 'number';
}

// 13. Creating enums

enum Colors { Red, Green, Blue }

// 14. Writing classes

class Vector {
    constructor(public x: number, public y: number) {
        // ...
    }

    add(vec: Vector) {
        this.x += vec.x;
        this.y += vec.y;
    }
}

// 15. Defining functions

function addNumbers(a: number, b: number): number {
    return a + b;
}

// 16. Understanding scope

const myGlobal = 12;

function outerScope() {
    const myLocal = 3;
    console.log(myGlobal); // 12 
    console.log(myLocal); // 3
}

// 17. Working with strings

let str1 = 'Hello, ';
let str2 = 'world!';
let str3 = str1.concat(str2);

// 18. Managing exceptions

try {
    throw new Error('My error message');
} catch (e) {
    console.log(e.message); 
}

// 19. Debugging

console.log('Arguments:', process.argv);

// 20. Optimizing

function isPrime(num: number): boolean {
    let isPrime = true;
    for (let i = 2; i < num; i++) {
        if (num % i === 0) {
            isPrime = false;
            break; 
        }
    }
    return isPrime;
}