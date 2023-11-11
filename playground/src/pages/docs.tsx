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
      <div className="vstack gap-4">
        <DocCard
          title="Variables"
          anchor="variables"
          code={['var value = 123;', 'print value; // out: 123']}
          height="50px"
        />

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
        />

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
        />

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
