/* eslint-disable @typescript-eslint/no-empty-function */
/* eslint-disable no-restricted-globals */
import React from 'react';

import { Link } from 'react-router-dom';
import { Example } from '../components/example';
import { Navbar } from '../components/navbar';

interface DocCardProps {
  title: string;
  code: string | string[];
  playgroundCode: string;
  height: string;
}

const DocCard: React.FC<DocCardProps> = ({
  title,
  code,
  playgroundCode,
  height,
}) => (
  <div className="card p-2">
    <h2 className="fs-3">{title}</h2>
    <Example height={height}>{code}</Example>
    <Link to={`/?code=${playgroundCode}`}>Playground</Link>
  </div>
);

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
          code={['var value = 123;', 'print value; // out: 123']}
          playgroundCode="G4QwTgBKA2CuCmEC8ECMAmAzAbgFAAcwBLAOwBcoQ55sIB6OiAe1jIC40sg"
          height="50px"
        />

        <DocCard
          title="Functions"
          code={[
            'fn sum (a, b) {',
            '  return a + b;',
            '}',
            '',
            'print sum(60, 40); // out: 100',
          ]}
          playgroundCode="GYOwBAzgrgtmAUBDANGARgSjAbwFBjACcBTAFykPETAGp0BuXAX11wAdCBLEUyWeAGwAGVABYhGemAD00sAHsopAFxgAjEKFA"
          height="100px"
        />

        <DocCard
          title="String Concatenation"
          code={[
            'fn hello(name) {',
            '  return "Hello " + name;',
            '}',
            '',
            'print hello("World"); // out: Hello World',
          ]}
          playgroundCode="GYOwBAFgpgNjD2AKEBDAtlAlGA3gKDDACcoAXAVyPACIAJWBMasAajFQwG48BfPPAA5EAliFKQGSagHV4RGABNqmTmAD0asPHKkAXGHpx4YWfIVA"
          height="100px"
        />

        <DocCard
          title="Closures"
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
          playgroundCode="GYOwBA5gTgpjAuAKad4EsQQJRgN4CgwxQwAHGKAZwHsREQBDAWxhwKKNngFcpwUEGCGADUYAEQTRYRiwDchMAF98irr3DkqtBSvykoGeJFgJE4gBIwANteris5gOrUo1gCYO5YAPQ+w1NzwAFxgVrbUYC5u7kA"
          height="200px"
        />

        <DocCard
          title="For Loops"
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
          playgroundCode="GYewTgBAFAbghpAlhAvBADAbgsgPBARix1RIGpCBKCAbwCgIIAHMRAOwBcdM6BfOugHpBEEAFcOALgxCR4qYVmiJ0gExL50gMwaVEACy6FAViPSAbGYgB2KwA4rATiA"
          height="300px"
        />

        <DocCard
          title="If/Else"
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
          playgroundCode="G4QwTgBAlgzgKmArgUwgXggFycg3AKHygDMIAKWBFASggG98IIAHMKAO0wgCIB1EGFhwBCbgQC+EZABsYqBk1YcufARGIhZyURMIB6PRAD2iTAC4I-QdhTCgA"
          height="200px"
        />

        <DocCard
          title="Classes"
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
          playgroundCode="MYGwhgzhAEDiBOBTRAXR9oG8BQ1oEsA7fFACgHMlUjyBKLXPaFAC3wgDpLkUboBeaN2qFyAbkYBfbI2FlCYALaJ6OJtCQoArvELM2nOXwDU0AETnophcol5p07ADcwGOegFwqaeKTMAJRBAQAHszWglsAAd4IhQhb3Qubz8AdRD4EAATcLFoAHp86BCtFAAuaEDgkOh0zKygA"
          height="300px"
        />

        <DocCard
          title="Inheritance"
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
          playgroundCode="MYGwhgzhAEDiBOBTRAXR9oG8BQ1oEsA7fFACgHMlUjyBKLXPaFAC3wgDpLkUboBeaN2qFyAbkYBfbI2FlCYALaJ6OJtCQoArvELM2nOXwDU0AETnophcol5p07KEgwAEohAgA9gh7poADxwVGgYagTEZKqMeBBaAA7oHEQkpGbunl5mtHbQDjIAbmAYcv6CGd6+qOikOTLx8EQoQiFJcmkA6l7wIAAm2WLQAPRD0F5aKABc0BVe0F09vUA"
          height="400px"
        />
      </div>
    </div>
  </>
);

export default Docs;
