/* eslint-disable @typescript-eslint/no-empty-function */
/* eslint-disable no-restricted-globals */
import React from 'react';

import { Navbar } from '../components/navbar';

/**
 * Locks's tooling page component
 * @returns A page component
 */
const Tooling: React.FC = () => (
  <>
    <Navbar />
    <div className="m-4">
      <div className="vstack gap-4">
        <h2>
          <span
            className="me-2 bi bi-wrench text-black align-text-bottom"
            role="img"
            aria-hidden="true"
          />
          Language Tooling
        </h2>
        <div className="shadow rounded p-3">
          <h3>
            <span
              className="me-2 bi bi-puzzle text-black align-text-bottom"
              role="img"
              aria-hidden="true"
            />
            VS Code Extension
          </h3>
          <p>
            See:{' '}
            <a
              href="https://github.com/kyleect/locks#vs-code-extension"
              target="_blank"
              rel="noreferrer"
            >
              https://github.com/kyleect/locks#vs-code-extension
            </a>
          </p>

          <h4>Download</h4>

          <ol>
            <li>
              Download extension from latest passing{' '}
              <a
                href="https://github.com/kyleect/locks/actions/workflows/build-artifacts.yml"
                target="_blank"
                rel="noreferrer"
              >
                build
              </a>
              .
            </li>
          </ol>

          <h4>Or Build & Install</h4>

          <ol>
            <li>Clone repo</li>
            <li>
              Run <code>just build-all</code>
            </li>
            <li>
              Install extension from:{' '}
              <code>vsc/out/locks-language-1.0.0.vsix</code>
            </li>
          </ol>
        </div>

        <div className="shadow rounded p-3">
          <h3>
            <span
              className="me-2 bi bi-stack text-black align-text-bottom"
              role="img"
              aria-hidden="true"
            />
            Docker
          </h3>

          <p>
            See:{' '}
            <a
              href="https://github.com/kyleect/locks#docker"
              target="_blank"
              rel="noreferrer"
            >
              https://github.com/kyleect/locks#docker
            </a>
          </p>

          <ol>
            <li>Clone repo</li>
            <li>
              Run <code>docker build -t kyleect/locks:1.0.0 .</code>
            </li>
            <li>
              Run{' '}
              <code>docker run --rm -it kyleect/locks:1.0.0 locks repl</code>
            </li>
          </ol>
        </div>
      </div>
    </div>
  </>
);

export default Tooling;
