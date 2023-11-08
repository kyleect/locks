/* eslint-disable @typescript-eslint/no-empty-function */
/* eslint-disable no-restricted-globals */
import React from 'react';

import { Example } from '../components/example';
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
        <div className="card p-2">
          <h2 className="fs-3">Variables</h2>

          <div style={{ height: 50 }}>
            <Example>
              {['var value = 123;', 'print value; // out: 123']}
            </Example>
          </div>

          <a href="/#/?code=code=G4QwTgBKA2CuCmEC8ECMAmAzAbgLACgAHMASwDsAXKEOebCAegYgHtYKAuNLIA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">Functions</h2>

          <div style={{ height: 100 }}>
            <Example>
              {[
                'fn sum (a, b) {',
                '  return a + b;',
                '}',
                '',
                'print sum(60, 40); // out: 100',
              ]}
            </Example>
          </div>

          <a href="/#/?code=GYOwBAzgrgtmAUBDANGARgSjAbwLACgwwAnAUwBcpjxEwBqdAbgIF8CCAHYgSxHMljwAbAAZUAFhEZGYAPSywAeyjkAXGACMIkUA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">String Concatenation</h2>

          <div style={{ height: 100 }}>
            <Example>
              {[
                'fn hello(name) {',
                '  return "Hello " + name;',
                '}',
                '',
                'print hello("World"); // out: Hello World',
              ]}
            </Example>
          </div>

          <a href="/#/?code=GYOwBAFgpgNjD2AKEBDAtlAlGA3gWACgwwAnKAFwFcTwAiACVgTFrAGoxUMBuQgX0KEADiQCWIcpCZJaAdXgkYAE1qZuYAPQaw8SuQBcYRnHhh5ipUA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">Closures</h2>

          <div style={{ height: 200 }}>
            <Example>
              {[
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
            </Example>
          </div>

          <a href="/#/?code=GYOwBA5gTgpjAuAKad4EsQQJRgN4FgAoMMUMABxigGcB7EREAQwFsYcDiSxZ4BXKOBQIMEMAGowAImkSwzNgG4iJAL4qwG3gPCUa9ZYXWEi5KBniRYCRFIASMADaPaUrLYDqtKI4AmbxTAAeiCwWj54AC4wB2daMC8fXyA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">For Loops</h2>

          <div style={{ height: 300 }}>
            <Example>
              {[
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
            </Example>
          </div>

          <a href="/#/?code=GYewTgBAFAbghpAlhAvBADAbgsgPBARix1RIGpCBKCAbwFgAoCCABzEQDsAXHTRgX0ZA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">If/Else</h2>

          <div style={{ height: 200 }}>
            <Example>
              {[
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
            </Example>
          </div>

          <a href="/#/?code=G4QwTgBAlgzgKmArgUwgXggFycg3AWACgioAzCAClgRQEoIBvIiCABzCgDtMIAiAdRAwsOAIS8ChAL4RkAGxiomhFuy48BQiKRALk4yVKJEA9CYgB7RJgBcEQcOwpRQAA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">Classes</h2>

          <div style={{ height: 300 }}>
            <Example>
              {[
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
            </Example>
          </div>

          <a href="/#/?code=MYGwhgzhAEDiBOBTRAXR9oG8CwAoa0AlgHaEoAUA5kqiZQJRZ4EEoAWhEAdNcindAC80XrWKUA3M2gBfPNNEViYALaJGOfCyQoArvGLR2nHjX7joAamgAiW1ejK1UrXNxu8ANzAZF6IXBm6OQ2ABKIICAA9jb0LngADvAkKCJB8KZ8IQDqUfAgACaxEtAA9KXQUbooAFzQ4ZFR0Ln5BXhAA">
            Playground
          </a>
        </div>
        <div className="card p-2">
          <h2 className="fs-3">Inheritance</h2>

          <div style={{ height: 400 }}>
            <Example>
              {[
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
            </Example>
          </div>

          <a href="/#/?code=MYGwhgzhAEDiBOBTRAXR9oG8CwAoa0AlgHaEoAUA5kqiZQJRZ4EEoAWhEAdNcindAC80XrWKUA3M2gBfPNNEViYALaJGOfCyQoArvGLR2nHjX7joAamgAiW1ejK1UrXNxu8oSDAASiECAA9gh86NAAPHBmYZoEJGTkGtIEELoADuhc8RQ2fgGBNvQuBG4euABuYBiKYcJ5QSGo6IkueGnwJCgi0fCmfOQ2AOqB8CAAJoUS0AD009CBuigAXND1gdDDo2N4QA">
            Playground
          </a>
        </div>
      </div>
    </div>
  </>
);

export default Docs;
