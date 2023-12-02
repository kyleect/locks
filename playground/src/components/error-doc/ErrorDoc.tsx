import React from 'react';
import { Link } from 'react-router-dom';

interface ErrorDocProps {
  errorName: string;
  id: string;
  signature: string;
  description: React.ReactNode;
}

const ErrorDoc: React.FC<ErrorDocProps> = ({
  errorName,
  id,
  signature,
  description,
}) => (
  <div>
    <h4 id={id}>
      {errorName}{' '}
      <Link to={`#${id}`}>
        <span
          className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
          role="img"
          aria-hidden="true"
        />
      </Link>{' '}
      <code>{signature}</code>
    </h4>
    <p>{description}</p>
  </div>
);

export { ErrorDocProps, ErrorDoc };
