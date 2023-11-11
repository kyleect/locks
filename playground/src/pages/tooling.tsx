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
      <div className="vstack gap-5">
        <h2>
          <span
            className="me-2 bi bi-wrench text-black align-text-bottom"
            role="img"
            aria-hidden="true"
          />
          Language Tooling
        </h2>
        <div>
          <h3>
            <span
              className="me-2 bi bi-puzzle text-black align-text-bottom"
              role="img"
              aria-hidden="true"
            />
            VS Code Extension
          </h3>
          See:{' '}
          <a
            href="https://github.com/kyleect/locks#vs-code-extension"
            target="_blank"
            rel="noreferrer"
          >
            https://github.com/kyleect/locks#vs-code-extension
          </a>
        </div>
        <div>
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
        </div>
      </div>
    </div>
  </>
);

export default Tooling;
