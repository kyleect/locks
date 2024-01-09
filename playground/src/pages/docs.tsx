/* eslint-disable @typescript-eslint/no-empty-function */
/* eslint-disable no-restricted-globals */
import React from 'react';

import { Link } from 'react-router-dom';
import { DocCard } from '../components/doc-card';
import { ErrorDoc } from '../components/error-doc';
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
            <Link className="nav-link" to="#errors">
              Errors
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
            <Link className="nav-link" to="#functions-single-expression-bodies">
              Single Expression Function Bodies
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
            <Link className="nav-link" to="#classes-inheritance">
              Class Inheritance
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#classes-object">
              Object Base Class
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#classes-index-access">
              Class Field/Method Index Access
            </Link>
          </li>
          <li className="nav-item">
            <Link
              className="nav-link"
              to="#classes-single-expression-method-bodies"
            >
              Single Expression Method Bodies
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#lists ">
              Lists
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#typeof ">
              typeof
            </Link>
          </li>
          <li className="nav-item">
            <Link className="nav-link" to="#instanceof ">
              instanceof
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
            '        println("FizzBuzz");',
            '      }',
            '      else if (i % 3 == 0) {',
            '        println("Fizz");',
            '      }',
            '      else if (i % 5 == 0) {',
            '        println("Buzz");',
            '      }',
            '      else {',
            '        println(i);',
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
            'println(value); // out: nil',
            'value = 42;',
            'println(value); // out: 42;',
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
            'println(nil); // out: nil',
            '',
            'fn noReturn() {}',
            'println(noReturn()); // out: nil',
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
            'println(123); // out: 123',
            'println(123.45); // out: 123.45',
            'println(-123); // out: -123',
            'println(-123.45); // out: -123.45',
            'println((5 + 7) * 2.5); // out: 30;',
            'println(5 % 1); // out: 0',
            'println(5 % 2); // out: 1',
            'println(5 % 3); // out: 2',
            'println(5 % 4); // out: 1',
            'println(5 % 5); // out: 0',
          ]}
          height="300px"
        >
          A 64bit float value.
        </DocCard>
        <DocCard
          title="Booleans"
          anchor="booleans"
          code={[
            'println(true); // out: true',
            'println(true and false); // out: false',
            'println(true or false); // out: true',
            'println(!true); // out: false',
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
            '  println("Was true!");',
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
            '  println("Was true!");',
            '} else {',
            '  println("Was false!");',
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
            'println("Hello World"); // out: Hello World',
            'println(len("Hello World")); // out: 11',
          ]}
          height="50px"
        >
          Strings are created using double quotes.
        </DocCard>
        <DocCard
          title="String Concatenation"
          anchor="string-concatenation"
          code={['println("Hello" +" "+ "World"); // out: Hello World']}
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
            'println(sum(60, 40)); // out: 100',
          ]}
          height="100px"
        />
        <DocCard
          title="Single Expression Function Bodies"
          anchor="functions-single-expression-bodies"
          code={[
            'fn sum (a, b) => a + b;',
            '',
            'println(sum(60, 40)); // out: 100',
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
            'println(add(70, 20)); // out: 90',
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
            'println(greet("Hello")("World")); // out: Hello World',
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
            '  println(i);',
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
            '  println(a);',
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
            '  let greeting;',
            '  let flair = "!!!";',
            '',
            '  fn init(greeting) {',
            '    this.greeting = greeting;',
            '  }',
            '',
            '  fn greet(name) {',
            '    return this.greeting + " " + name + this.flair;',
            '  }',
            '}',
            '',
            'let greeter = Greeter("Hello");',
            '',
            'println(greeter.greet("World")); // out: Hello World!!!',
          ]}
          height="325px"
        />
        <DocCard
          title="Class Inheritance"
          anchor="classes-inheritance"
          code={[
            'class Greeter {',
            '  let greeting;',
            '  let flair = "!!!";',
            '',
            '  fn init(greeting) {',
            '    this.greeting = greeting;',
            '  }',
            '',
            '  fn greet(name) {',
            '    return this.greeting + " " + name + this.flair;',
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
            'println(greeter.greet("World")); // out: Hello World!!!',
          ]}
          height="425px"
        >
          Classes can extend other classes. All classes extend or decend from
          the base class `Object`.
        </DocCard>

        <DocCard
          title="Object Base Class"
          anchor="classes-object"
          code={[
            'class Example {}',
            'let example = Example();',
            'println(instanceof(example, Example)); // out: true',
            'println(instanceof(example, Object)); // out: true',
          ]}
          height="100px"
        >
          All classes extend or descend from the base class `Object`.
        </DocCard>

        <DocCard
          title="Class Field/Method Index Access"
          anchor="classes-index-access"
          code={[
            'class Box {',
            '  let value;',
            '',
            '  fn init(value) {',
            '    this["value"] = value;',
            '  }',
            '',
            '  fn get() {',
            '    return this["value"];',
            '  }',
            '}',
            '',
            'let box = Box(123);',
            '',
            'println(box.get()); // out: 123',
          ]}
          height="300px"
        />

        <DocCard
          title="Single Expression Method Bodies"
          anchor="classes-single-expression-method-bodies"
          code={[
            'class Box {',
            '  let value;',
            '',
            '  fn init(value) {',
            '    this.value = value;',
            '  }',
            '',
            '  fn get() => this.value;',
            '}',
            '',
            'let box = Box("Hello");',
            '',
            'println(box.get()); // out: Hello',
            '',
            'box.value = "World";',
            '',
            'println(box.get()); // out: World',
          ]}
          height="350px"
        />

        <DocCard
          title="Lists"
          anchor="lists"
          code={[
            'let list = [1, 2, 3];',
            '',
            'list[5] = 100;',
            '',
            'println(list); // out: [1, 2, 3, nil, nil, 100]',
            'println(len(list)); // out: 6',
          ]}
          height="150px"
        >
          Lists can store a dynamic number of mixed types in a collection.
        </DocCard>

        <DocCard
          title="typeof"
          anchor="typeof"
          code={[
            'println(typeof(123)); // out: number',
            'println(typeof("Hello")); // out: string',
            '',
            'println(typeof(println)); // out: function',
            '',
            'fn sum (a, b) => a + b;',
            'println(typeof(sum)); // out: function',
            '',
            'println(typeof(true)); // out: boolean',
            'println(typeof(false)); // out: boolean',
            '',
            'class Example {',
            '  fn test() => 123;',
            '}',
            'println(typeof(Example)); // out: class',
            '',
            'let example = Example();',
            'println(typeof(example)); // out: instance',
            'println(typeof(example.test)); // out: function',
            '',
            'println(typeof([1, 2, 3])); // out: list',
            'println(typeof(nil)); // out: nil',
          ]}
          height="450px"
        >
          Return a value&apos;s type as a string: <code>number</code> |{' '}
          <code>string</code> | <code>function</code> | <code>boolean</code> |{' '}
          <code>list</code> | <code>nil</code>
        </DocCard>

        <DocCard
          title="instanceof"
          anchor="instanceof"
          code={[
            'class GrandParent {}',
            '',
            'class Parent extends GrandParent {}',
            '',
            'class Child extends Parent {}',
            '',
            'class OtherChild extends Parent {}',
            '',
            'class Grandchild extends Child {}',
            '',
            'let instance = Grandchild();',
            '',
            'println(instanceof(instance, Grandchild)); // out: true',
            'println(instanceof(instance, Child)); // out: true',
            'println(instanceof(instance, Parent)); // out: true',
            'println(instanceof(instance, GrandParent)); // out: true',
            'println(instanceof(instance, OtherChild)); // out: false',
          ]}
          height="400px"
        >
          Return <code>bool</code> if the value is an instance of the
          class/super class.
        </DocCard>

        <div className="shadow rounded p-3 vstack gap-3">
          <h2 id="errors">
            Errors{' '}
            <Link to="#errors">
              <span
                className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                role="img"
                aria-hidden="true"
              />
            </Link>
          </h2>
          <ul className="nav">
            <li className="nav-item">
              <Link className="nav-link" to="#errors--attribute-error">
                AttributeError
              </Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#errors--io-error">
                IoError
              </Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#errors--name-error">
                NameError
              </Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#errors--overflow-error">
                OverflowError
              </Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#errors--syntax-error">
                SyntaxError
              </Link>
            </li>
            <li className="nav-item">
              <Link className="nav-link" to="#errors--type-error">
                TypeError
              </Link>
            </li>
          </ul>

          <div className="vstack gap-3">
            <h3 id="errors--attribute-error">
              AttributeError{' '}
              <Link to="#errors--attribute-error">
                <span
                  className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                  role="img"
                  aria-hidden="true"
                />
              </Link>
            </h3>

            <ErrorDoc
              errorName="NoSuchAttribute"
              signature="(_type: String, name: String)"
              description={
                <>
                  Instance of <code>type_</code> has no such attribute{' '}
                  <code>name</code>.
                </>
              }
              id="errors--attribute-error--no-such-attribute"
            />

            <ErrorDoc
              errorName="NoSuchField"
              signature="(class_name: String, field_name: String)"
              description={
                <>
                  Class has no such field <code>field_name</code>.
                </>
              }
              id="errors--attribute-error--no-such-field"
            />
          </div>

          <div className="vstack gap-3">
            <h3 id="errors--io-error">
              IoError{' '}
              <Link to="#errors--attribute-error">
                <span
                  className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                  role="img"
                  aria-hidden="true"
                />
              </Link>
            </h3>

            <ErrorDoc
              errorName="WriteError"
              signature="(file: String)"
              description={
                <>
                  Unable to write to file at <code>path</code>.
                </>
              }
              id="errors--io-error--write-error"
            />
          </div>

          <div className="vstack gap-3">
            <h3 id="errors--name-error">
              NameError{' '}
              <Link to="#errors--attribute-error">
                <span
                  className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                  role="img"
                  aria-hidden="true"
                />
              </Link>
            </h3>

            <ErrorDoc
              errorName="AccessInsideInitializer"
              signature="(name: String)"
              description={
                <p>
                  A variable referred to itself by <code>name</code> in it&#39;s
                  initializer.
                </p>
              }
              id="errors--name-error--access-inside-initializer"
            />

            <ErrorDoc
              errorName="AlreadyDefined"
              signature="(name: String)"
              description={
                <p>
                  An already defined <code>name</code> was used for a
                  declaration.
                </p>
              }
              id="errors--name-error--already-defined"
            />

            <ErrorDoc
              errorName="ClassInheritFromSelf"
              signature="(name: String)"
              description={<p>Class tried to extend itself.</p>}
              id="errors--name-error--class-inherit-from-self"
            />

            <ErrorDoc
              errorName="NotDefined"
              signature="(name: String)"
              description={
                <p>
                  An undefined <code>name</code> was referenced.
                </p>
              }
              id="errors--name-error--not-defined"
            />

            <ErrorDoc
              errorName="ReservedName"
              signature="(name: String)"
              description={
                <p>
                  A reserved <code>name</code> was used in a declaration.
                </p>
              }
              id="errors--name-error--reserved-name"
            />
          </div>

          <div className="vstack gap-3">
            <h3 id="errors--overflow-error">
              OverflowError{' '}
              <Link to="#errors--attribute-error">
                <span
                  className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                  role="img"
                  aria-hidden="true"
                />
              </Link>
            </h3>

            <ErrorDoc
              errorName="JumpTooLarge"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--jump-too-large"
            />

            <ErrorDoc
              errorName="StackOverflow"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--stackoverflow"
            />

            <ErrorDoc
              errorName="TooManyArgs"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--too-many-args"
            />

            <ErrorDoc
              errorName="TooManyConstants"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--too-many-constants"
            />

            <ErrorDoc
              errorName="TooManyLocals"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--too-many-locals"
            />

            <ErrorDoc
              errorName="TooManyParams"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--too-many-params"
            />

            <ErrorDoc
              errorName="TooManyUpvalues"
              signature=""
              description={<p> </p>}
              id="errors--overflow-error--too-many-upvalues"
            />
          </div>

          <div className="vstack gap-3">
            <h3 id="errors--syntax-error">
              SyntaxError{' '}
              <Link to="#errors--attribute-error">
                <span
                  className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                  role="img"
                  aria-hidden="true"
                />
              </Link>
            </h3>

            <ErrorDoc
              errorName="ExtraToken"
              signature="(token: String)"
              description={<p> </p>}
              id="errors--syntax-error--extra-token"
            />

            <ErrorDoc
              errorName="InvalidToken"
              signature=""
              description={<p> </p>}
              id="errors--syntax-error--invalid-token"
            />

            <ErrorDoc
              errorName="ReturnInInitializer"
              signature=""
              description={<p> </p>}
              id="errors--syntax-error--return-in-initializer"
            />

            <ErrorDoc
              errorName="ReturnOutsideFunction"
              signature=""
              description={
                <p>
                  <code>this</code> was used outside of a function.
                </p>
              }
              id="errors--syntax-error--return-outside-function"
            />

            <ErrorDoc
              errorName="SuperOutsideClass"
              signature=""
              description={
                <p>
                  <code>this</code> was used outside of a class.
                </p>
              }
              id="errors--syntax-error--super-outside-class"
            />

            <ErrorDoc
              errorName="SuperWithoutSuperclass"
              signature=""
              description={
                <p>
                  Super referenced in a class that doesn&#39;t extend another.
                  class
                </p>
              }
              id="errors--syntax-error--super-without-super-class"
            />

            <ErrorDoc
              errorName="ThisOutsideClass"
              signature=""
              description={
                <p>
                  <code>this</code> was used outside of a class.
                </p>
              }
              id="errors--syntax-error--this-outside-class"
            />

            <ErrorDoc
              errorName="UnexpectedInput"
              signature="(token: String)"
              description={<p> </p>}
              id="errors--syntax-error--unexpected-input"
            />

            <ErrorDoc
              errorName="UnrecognizedEOF"
              signature="(token: String)"
              description={<p> </p>}
              id="errors--syntax-error--unrecognized-eof"
            />

            <ErrorDoc
              errorName="UnrecognizedToken"
              signature="(token: String, expected: String[])"
              description={
                <p>
                  An unrecognized <code>token</code> was found during parsing.
                </p>
              }
              id="errors--syntax-error--unrecognized-token"
            />

            <ErrorDoc
              errorName="UnterminatedString"
              signature=""
              description={
                <p>
                  A string is missing an end double quote <code>&ldquo;</code>
                </p>
              }
              id="errors--syntax-error--unterminated-string"
            />
          </div>

          <div className="vstack gap-3">
            <h3 id="errors--type-error">
              TypeError{' '}
              <Link to="#errors--type-error">
                <span
                  className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
                  role="img"
                  aria-hidden="true"
                />
              </Link>
            </h3>

            <ErrorDoc
              errorName="ArityMismatch"
              signature="(name: String, exp_args: Number, got_args: Number)"
              description={
                <p>
                  Function or method called with incorrect number of arguments.
                </p>
              }
              id="errors--type-error--arity-mismatch"
            />

            <ErrorDoc
              errorName="InitInvalidReturnType"
              signature="(type_: String)"
              description={
                <p>
                  Invalid return value from classes `init` constructor method.
                </p>
              }
              id="errors--type-error--init-invalid-return-type"
            />

            <ErrorDoc
              errorName="NotCallable"
              signature="(type_: String)"
              description={<p>A non function or method was called.</p>}
              id="errors--type-error--not-callable"
            />

            <ErrorDoc
              errorName="SuperclassInvalidType"
              signature="(type_: String)"
              description={<p>Class extends non class type.</p>}
              id="errors--type-error--superclass-invalid-type"
            />

            <ErrorDoc
              errorName="UnsupportedOperandInfix"
              signature="(op: String, lt_type: String, rt_type: String)"
              description={
                <p>
                  An infix operator (e.g. <code>+</code>, <code>-</code>) used
                  on invalid type/s.
                </p>
              }
              id="errors--type-error--unsupported-operand-infix"
            />

            <ErrorDoc
              errorName="UnsupportedOperandPrefix"
              signature="(op: String, rt_type: String)"
              description={
                <p>
                  An prefix operator (e.g. <code>-</code>) used on an invalid
                  type.
                </p>
              }
              id="errors--type-error--unsupported-operand-prefix"
            />

            <ErrorDoc
              errorName="InvalidMethodAssignment"
              signature="(name: String, type_: String)"
              description={<p>A method was re/assigned on a class instance.</p>}
              id="errors--type-error--invalid-method-assignment"
            />
          </div>
        </div>

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
                  href="https://github.com/kyleect/locks/actions/workflows/build-artifacts.yml"
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
            <pre>$ locks exec &squo;println(&quot;Hello&quot;;&squo);</pre>
            <pre>$ cat res/benchmarks/fib.locks | locks exec</pre>
          </div>
          <div className="shadow rounded p-3">
            <h4>Parse File</h4>
            Visualize AST from file
            <pre>$ locks parse ./res/examples/number/fizzbuzz.locks</pre>
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
