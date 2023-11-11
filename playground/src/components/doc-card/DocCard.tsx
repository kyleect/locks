import React, { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import {
  compressToEncodedURIComponent,
  // eslint-disable-next-line import/no-relative-packages
} from '../../../vendor/lz-string';
import { useLocks } from '../../hooks/useLocks';
import { Example } from '../example';
import { LocksRunButton } from '../locks-run-button';
import { Output } from '../output';

interface DocCardProps {
  title: string;
  anchor: string;
  code: string | string[];
  height: string;
  children?: ReactNode;
}
const DocCard: React.FC<DocCardProps> = ({
  title,
  code,
  height,
  anchor,
  children,
}) => {
  const { isRunning, runLocks, stopLocks, locksResult } = useLocks();
  const value = Array.isArray(code) ? code.join('\n') : code;

  return (
    <div className="shadow p-3 rounded">
      <h2 className="fs-4">
        {title}{' '}
        <Link to={`#${anchor}`}>
          <span
            className="me-1 bi bi-link-45deg link-secondary align-text-bottom"
            role="img"
            aria-hidden="true"
          />
        </Link>
        <Link
          to={`/?code=${compressToEncodedURIComponent(value)}`}
          title="Open this code in the playground"
        >
          <span
            className="fs-5 bi bi-rocket-takeoff link-primary"
            role="img"
            aria-hidden="true"
          />
        </Link>
      </h2>

      {children ? <p>{children}</p> : null}

      <div className="card p-2" id={anchor}>
        <Example height={height}>{code}</Example>

        <LocksRunButton
          isRunning={isRunning}
          onClick={isRunning ? stopLocks : () => runLocks(value)}
        />
        {typeof locksResult === 'string' && (
          <>
            <h3 className="fs-5 mt-2">Result:</h3>
            <div className="card bg-light bg-gradient">
              <Output text={locksResult} />
            </div>
          </>
        )}
      </div>
    </div>
  );
};

DocCard.defaultProps = {
  children: '',
};

export { DocCardProps, DocCard };
