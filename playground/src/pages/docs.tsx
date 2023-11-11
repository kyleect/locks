/* eslint-disable @typescript-eslint/no-empty-function */
/* eslint-disable no-restricted-globals */
import React from 'react';

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
      <div className="vstack gap-5">
        <h2>
          <span
            className="me-2 bi bi-code-slash text-black align-text-bottom"
            role="img"
            aria-hidden="true"
          />{' '}
          Documentation
        </h2>
        <DocCard
          title="Comments"
          anchor="comments"
          code="// Line comments are supported"
          height="30px"
        />

        <DocCard
          title="Variables"
          anchor="variables"
          code={['var value;', '', 'print value; // nil']}
          height="70px"
        >
          Variables are declared using the <code>var</code> keyword with an
          identifier/name matching <code>[a-zA-Z_][a-zA-Z0-9_]*</code>.
          Supported value types: string, number, boolean, function,{' '}
          <code>nil</code>. Variables default to <code>nil</code>.
        </DocCard>

        <DocCard
          title="Nil"
          anchor="nil"
          code={['var value = nil;', '', 'print nil; // out: nil']}
          height="70px"
        >
          A null value.
        </DocCard>

        <DocCard
          title="Numbers"
          anchor="numbers"
          code={['var value = 123;', '', 'print value; // out: 123']}
          height="70px"
        />

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
            'var isTrue = true;',
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
            'var isTrue = true;',
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
          code={[
            'var value = "Hello World";',
            '',
            'print value; // out: Hello World',
          ]}
          height="70px"
        >
          Strings are created using double quotes.
        </DocCard>

        <DocCard
          title="String Concatenation"
          anchor="string-concatenation"
          code={[
            'fn hello(name) {',
            '  return "Hello " + name;',
            '}',
            '',
            'print hello("World"); // out: Hello World',
          ]}
          height="100px"
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
          title="First Class Functions"
          anchor="functions"
          code={[
            'fn sum (a, b) {',
            '  return a + b;',
            '}',
            '',
            'var add = sum;',
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
            'for (var i = 0; i < 10; i = i + 1) {',
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
            'var a = 1;',
            'while (a < 10) {',
            '  print a;',
            '  a = a + 1;',
            '}',
          ]}
          height="110px"
        />

        <DocCard
          title="Classes"
          anchor="classes"
          code={[
            'class Greeter {',
            '  init(greeting) {',
            '    this.greeting = greeting;',
            '  }',
            '',
            '  greet(name) {',
            '    return this.greeting + " " + name;',
            '  }',
            '}',
            '',
            'var greeter = Greeter("Hello");',
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
            '  init(greeting) {',
            '    this.greeting = greeting;',
            '  }',
            '',
            '  greet(name) {',
            '    return this.greeting + " " + name;',
            '  }',
            '}',
            '',
            'class HelloGreeter < Greeter {',
            '  init() {',
            '    super.init("Hello");',
            '  }',
            '}',
            '',
            'var greeter = HelloGreeter();',
            '',
            'print greeter.greet("World"); // out: Hello World',
          ]}
          height="400px"
        />
      </div>
    </div>
  </>
);

export default Docs;
