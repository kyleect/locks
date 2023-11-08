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
  <nav className="navbar navbar-expand-lg p-1" id="navbar">
    <div className="container-fluid">
      <div className="navbar-brand fw-bold">
        <span className="bi bi-lock-fill" role="img" aria-hidden="true" />
        <span
          className="me-2 bi bi-unlock-fill"
          role="img"
          aria-hidden="true"
        />
        Locks {subBrandText}
      </div>
      <div className="d-flex">
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
        <div className="collapse navbar-collapse" id="navbarSupportedContent">
          <ul className="navbar-nav me-auto mb-2 mb-lg-0">
            <li className="nav-item nav-link">
              <NavLink
                className={({ isActive }) => (isActive ? 'fw-bold' : '')}
                aria-current="page"
                to="/"
              >
                Playground
              </NavLink>
            </li>
            <li className="nav-item nav-link">
              <NavLink
                className={({ isActive }) => (isActive ? 'fw-bold' : '')}
                aria-current="page"
                to="/docs"
              >
                Docs
              </NavLink>
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
      </div>
    </div>
  </nav>
);

export { Navbar, NavBarProps };
