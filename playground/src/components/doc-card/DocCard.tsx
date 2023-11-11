import React, { ReactNode } from 'react';
import { Link } from 'react-router-dom';
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
    <div>
      <h2 className="fs-4">
        {title}{' '}
        <Link to={`#${anchor}`}>
          <span
            className="me-2 bi bi-link-45deg text-black align-text-bottom"
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
