import React, { useEffect, useRef } from 'react';
import { Popover } from 'bootstrap';

interface NavBarProps {
  /**
   * Set to `true` if VM is currently running.
   */
  isRunning: boolean;
  onRunClick: () => void;
}

/**
 * Navbar component
 */
const Navbar = ({ onRunClick, isRunning }: NavBarProps) => {
  let runColor = 'btn-success';
  let runIcon = 'me-1 bi bi-play-fill';
  let runText = 'Run';

  if (isRunning) {
    runColor = 'btn-danger';
    runIcon = 'me-2 spinner-grow spinner-grow-sm';
    runText = 'Stop';
  }

  const clipboardRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    // eslint-disable-next-line no-new, @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-argument
    new Popover(clipboardRef.current as any, {
      content: 'Copied!',
      trigger: 'focus',
    });
  }, []);

  return (
    <nav className="navbar p-2" id="navbar">
      <div className="navbar-brand fw-bold">
        <span className="bi bi-lock-fill" role="img" aria-hidden="true" />
        <span
          className="me-2 bi bi-unlock-fill"
          role="img"
          aria-hidden="true"
        />
        Locks Playground
      </div>
      <div>
        <button
          className="btn btn-dark bi bi-github me-1"
          type="button"
          onClick={() => {
            window.open('https://github.com/kyleect/locks', '_blank');
          }}
          aria-label="Github repository"
        />
        <button
          className="btn btn-primary me-1"
          type="button"
          // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
          ref={clipboardRef}
          onClick={() => {
            navigator.clipboard.writeText(window.location.href).catch(() => {
              // eslint-disable-next-line no-alert
              alert('Unable to copy playground link');
            });
          }}
          aria-label="Github repository"
        >
          <span className="bi bi-clipboard" role="status" aria-hidden="true" />
        </button>
        <button
          id="run-btn"
          className={`btn ${runColor}`}
          onClick={onRunClick}
          type="button"
          aria-label="Run code"
        >
          <span className={runIcon} role="status" aria-hidden="true" />
          {runText}
        </button>
      </div>
    </nav>
  );
};

export { Navbar, NavBarProps };
