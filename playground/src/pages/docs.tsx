/* eslint-disable @typescript-eslint/no-empty-function */
/* eslint-disable no-restricted-globals */
import React from 'react';

import { Link } from 'react-router-dom';
import { DocCard } from '../components/doc-card';
import { Navbar } from '../components/navbar';

/**
 * Locks's documentation page component
 * @returns A page component
 */
const Docs: React.FC = () => (
  <>
    <Navbar subBrandText="Docs" />
    <div className="m-4">
      <div className="vstack gap-4">
        <h2>
          <span
            className="me-2 bi bi-code-square text-black align-text-bottom"
            role="img"
            aria-hidden="true"
          />{' '}
          Documentation
        </h2>

        <ul className="nav">
          <li className="nav-item">
            <Link className="nav-link" to="#syntax">
              Syntax
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#runtime">
              Runtime
            </Link>
          </li>
        </ul>

        <p>Welcome to the Locks language documentation.</p>

        <h2 id="syntax">
          Syntax{' '}
          <Link to="#syntax">
            <span
              className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
              role="img"
              aria-hidden="true"
            />
          </Link>
        </h2>

        <ul className="nav">
          <li className="nav-item">
            <Link className="nav-link" to="#example">
              Example
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#comments">
              Comments
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#variables">
              Variables
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#nil">
              Nil
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#numbers">
              Numbers
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#booleans">
              Booleans
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#if">
              If
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#if-else">
              If/Else
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#strings">
              Strings
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#string-concatenation">
              String Concatentation
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#functions">
              Functions
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#functions-as-values">
              Functions As Values
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#closures">
              Closures
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#for-loops">
              For Loops
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#while-loops">
              While Loops
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#classes">
              Classes
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#inheritance ">
              Inheritance
            </Link>
          </li>
        </ul>

        <DocCard
          title="Example"
          anchor="example"
          code={[
            'fn fizzBuzz(n) {',
            '  for (let i = 1; i <= n; i = i + 1) {',
            '      if (i % 15 == 0) {',
            '        print "FizzBuzz";',
            '      }',
            '      else if (i % 3 == 0) {',
            '        print "Fizz";',
            '      }',
            '      else if (i % 5 == 0) {',
            '        print "Buzz";',
            '      }',
            '      else {',
            '        print i;',
            '      }',
            '  }',
            '}',
            '',
            ' fizzBuzz(15);',
            '',
            '// out: 1',
            '// out: 2',
            '// out: Fizz',
            '// out: 4',
            '// out: Buzz',
            '// out: Fizz',
            '// out: 7',
            '// out: 8',
            '// out: Fizz',
            '// out: Buzz',
            '// out: 11',
            '// out: Fizz',
            '// out: 13',
            '// out: 14',
            '// out: FizzBuzz',
          ]}
          height="675px"
        />
        <DocCard
          title="Comments"
          anchor="comments"
          code="// Line comments are supported"
          height="30px"
        />
        <DocCard
          title="Variables"
          anchor="variables"
          code={[
            'let value;',
            'print value; // out: nil',
            'value = 42;',
            'print value; // out: 42;',
          ]}
          height="75px"
        >
          Variables are declared using the <code>let</code> keyword with an
          identifier/name matching <code>[a-zA-Z_][a-zA-Z0-9_]*</code>.
          Supported value types: string, number, boolean, function,{' '}
          <code>nil</code>. Variables default to <code>nil</code>. Variables can
          be reassigned.
        </DocCard>
        <DocCard
          title="Nil"
          anchor="nil"
          code={[
            'let value = nil;',
            'print nil; // out: nil',
            '',
            'fn noReturn() {}',
            'print noReturn(); // out: nil',
          ]}
          height="110px"
        >
          A null value. This is the default value for variables and functions
          without a return.
        </DocCard>
        <DocCard
          title="Numbers"
          anchor="numbers"
          code={[
            'print 123; // out: 123',
            'print 123.45; // out: 123.45',
            'print -123; // out: -123',
            'print -123.45; // out: -123.45',
            'print (5 + 7) * 2.5; // out: 30;',
            'print 5 % 1; // out: 0',
            'print 5 % 2; // out: 1',
            'print 5 % 3; // out: 2',
            'print 5 % 4; // out: 1',
            'print 5 % 5; // out: 0',
          ]}
          height="300px"
        >
          A 64bit float value.
        </DocCard>
        <DocCard
          title="Booleans"
          anchor="booleans"
          code={[
            'print true; // out: true',
            'print true and false; // out: false',
            'print true or false; // out: true',
            'print !true; // out: false',
          ]}
          height="100px"
        >
          <code>true</code> and <code>false</code> are booleans.{' '}
          <code>and</code>/<code>or</code> are logical operators. <code>!</code>{' '}
          negates a boolean.
        </DocCard>
        <DocCard
          title="If"
          anchor="if"
          code={[
            'let isTrue = true;',
            '',
            'if (isTrue) {',
            '  print "Was true!";',
            '}',
            '',
            '// out: Was true!',
          ]}
          height="150px"
        />
        <DocCard
          title="If/Else"
          anchor="if-else"
          code={[
            'let isTrue = true;',
            '',
            'if (isTrue) {',
            '  print "Was true!";',
            '} else {',
            '  print "Was false!";',
            '}',
            '',
            '// out: Was true!',
          ]}
          height="200px"
        />
        <DocCard
          title="Strings"
          anchor="strings"
          code={['print "Hello World"; // out: Hello World']}
          height="30px"
        >
          Strings are created using double quotes.
        </DocCard>
        <DocCard
          title="String Concatenation"
          anchor="string-concatenation"
          code={['print "Hello" +" "+ "World"; // out: Hello World']}
          height="30px"
        >
          Strings can be concatenated together using the <code>+</code>{' '}
          operator.
        </DocCard>
        <DocCard
          title="Functions"
          anchor="functions"
          code={[
            'fn sum (a, b) {',
            '  return a + b;',
            '}',
            '',
            'print sum(60, 40); // out: 100',
          ]}
          height="100px"
        />
        <DocCard
          title="Functions As Values"
          anchor="functions-as-values"
          code={[
            'fn sum (a, b) {',
            '  return a + b;',
            '}',
            '',
            'let add = sum;',
            '',
            'print add(70, 20); // out: 90',
          ]}
          height="150px"
        >
          Functions can be assign to variables, passed to and returned from
          other functions.
        </DocCard>
        <DocCard
          title="Closures"
          anchor="closures"
          code={[
            'fn greet(greeting) {',
            '  fn person(name) {',
            '    return greeting + " " + name;',
            '  }',
            '',
            '  return person;',
            '}',
            '',
            'print greet("Hello")("World"); // out: Hello World',
          ]}
          height="200px"
        >
          Functions can capture variables from their surrounding scope.
        </DocCard>
        <DocCard
          title="For Loops"
          anchor="for-loops"
          code={[
            'for (let i = 0; i < 10; i = i + 1) {',
            '  print i;',
            '}',
            '',
            '// out: 0',
            '// out: 1',
            '// out: 2',
            '// out: 3',
            '// out: 4',
            '// out: 5',
            '// out: 6',
            '// out: 7',
            '// out: 8',
            '// out: 9',
          ]}
          height="300px"
        />
        <DocCard
          title="While Loops"
          anchor="while-loops"
          code={[
            'let a = 1;',
            'while (a < 10) {',
            '  print a;',
            '  a = a + 1;',
            '}',
            '',
            '// out: 1',
            '// out: 2',
            '// out: 3',
            '// out: 4',
            '// out: 5',
            '// out: 6',
            '// out: 7',
            '// out: 8',
            '// out: 9',
          ]}
          height="300px"
        />
        <DocCard
          title="Classes"
          anchor="classes"
          code={[
            'class Greeter {',
            '  fn init(greeting) {',
            '    this.greeting = greeting;',
            '  }',
            '',
            '  fn greet(name) {',
            '    return this.greeting + " " + name;',
            '  }',
            '}',
            '',
            'let greeter = Greeter("Hello");',
            '',
            'print greeter.greet("World"); // out: Hello World',
          ]}
          height="300px"
        />
        <DocCard
          title="Inheritance"
          anchor="inheritance"
          code={[
            'class Greeter {',
            '  fn init(greeting) {',
            '    this.greeting = greeting;',
            '  }',
            '',
            '  fn greet(name) {',
            '    return this.greeting + " " + name;',
            '  }',
            '}',
            '',
            'class HelloGreeter extends Greeter {',
            '  fn init() {',
            '    super.init("Hello");',
            '  }',
            '}',
            '',
            'let greeter = HelloGreeter();',
            '',
            'print greeter.greet("World"); // out: Hello World',
          ]}
          height="400px"
        />

        <h2 id="runtime">
          Runtime{' '}
          <Link to="#runtime">
            <span
              className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
              role="img"
              aria-hidden="true"
            />
          </Link>
        </h2>

        <div className="vstack gap-4 ">
          <h3>Installing</h3>

          <div className="shadow rounded p-3">
            <ol>
              <li>
                Download runtime from latest passing{' '}
                <a
                  href="https://github.com/kyleect/locks/actions/workflows/ci.yml"
                  target="_blank"
                  rel="noreferrer"
                >
                  build
                </a>
                .
              </li>
              <li>
                Place <code>locks[.exe]</code> in your <code>PATH</code>.
              </li>
            </ol>
          </div>

          <h3>Usage</h3>

          <div className="shadow rounded p-3">
            <h4>Run A File</h4>
            <pre>$ locks run res/benchmarks/fib.locks</pre>
          </div>
          <div className="shadow rounded p-3">
            <h4>Run Input</h4>
            <pre>$ locks exec &squo;print &quot;Hello&quot;;&squo;</pre>
            <pre>$ cat res/benchmarks/fib.locks | locks exec</pre>
          </div>
          <div className="shadow rounded p-3">
            <h4>Disassemble File</h4>
            Visualize compiled byte code from file
            <pre>$ locks disassemble ./res/examples/number/fizzbuzz.locks</pre>
          </div>
          <div className="shadow rounded p-3">
            <h4>Start REPL</h4>
            <pre>$ locks repl</pre>
          </div>
        </div>
      </div>
    </div>
  </>
);

export default Docs;
