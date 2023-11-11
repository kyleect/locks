/* eslint-disable jsx-a11y/anchor-is-valid */
/* eslint-disable react/require-default-props */
import React, { ReactNode } from 'react';
import { NavLink } from 'react-router-dom';

interface NavBarProps {
  /**
   * Set to `true` if VM is currently running.
   */
  subBrandText: string;
  content?: ReactNode;
}

/**
 * Navbar component
 */
const Navbar = ({ subBrandText, content }: NavBarProps) => (
  <nav className="navbar navbar-dark bg-dark navbar-expand-lg p-1" id="navbar">
    <div className="container-fluid">
      <div className="navbar-brand fw-bold text-light">
        <span className="bi bi-lock-fill" role="img" aria-hidden="true" />
        <span
          className="me-2 bi bi-unlock-fill"
          role="img"
          aria-hidden="true"
        />
        Locks {subBrandText}
      </div>
      <div className="d-flex bg-dark">
        <div className="collapse navbar-collapse" id="navbarSupportedContent">
          <ul className="navbar-nav me-auto p-3 p-lg-0 mb-0">
            <li className="nav-item nav-link">
              <span
                className="me-2 bi bi-rocket-takeoff-fill text-white"
                role="img"
                aria-hidden="true"
              />
              <NavLink
                className={({ isActive }) => (isActive ? 'fw-bold' : '')}
                aria-current="page"
                to="/"
              >
                Playground
              </NavLink>
            </li>
            <li className="nav-item nav-link">
              <span
                className="me-2 bi bi-code-slash text-white"
                role="img"
                aria-hidden="true"
              />
              <NavLink
                className={({ isActive }) => (isActive ? 'fw-bold' : '')}
                aria-current="page"
                to="/docs"
              >
                Docs
              </NavLink>
            </li>
            <li className="nav-item nav-link">
              <span
                className="me-2 bi bi-terminal-fill text-white"
                role="img"
                aria-hidden="true"
              />
              <a
                href="https://github.com/kyleect/locks#vs-code-extension"
                target="_blank"
                rel="noreferrer"
              >
                VS Code
              </a>
            </li>
            <li className="nav-item">
              <button
                className="btn btn-dark bi bi-github me-1"
                type="button"
                onClick={() => {
                  window.open('https://github.com/kyleect/locks', '_blank');
                }}
                aria-label="Github repository"
              />
              {content}
            </li>
          </ul>
        </div>
        <button
          className="navbar-toggler"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#navbarSupportedContent"
          aria-controls="navbarSupportedContent"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          <span className="navbar-toggler-icon" />
        </button>
      </div>
    </div>
  </nav>
);

export { Navbar, NavBarProps };
