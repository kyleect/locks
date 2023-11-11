import React from 'react';
import { Link } from 'react-router-dom';
import { useLocks } from '../../hooks/useLocks';
import { Example } from '../example';
import { LocksRunButton } from '../locks-run-button';
import { Output } from '../output';

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
}) => {
  const { isRunning, runLocks, stopLocks, locksResult } = useLocks();
  const value = Array.isArray(code) ? code.join('\n') : code;

  return (
    <div className="card p-2">
      <h2 className="fs-3">{title}</h2>
      <Example height={height}>{code}</Example>
      <LocksRunButton
        isRunning={isRunning}
        onClick={isRunning ? stopLocks : () => runLocks(value)}
      />
      {typeof locksResult === 'string' && (
        <>
          <h3 className="fs-5">Output</h3>
          <div className="card">
            <Output text={locksResult} />
          </div>
        </>
      )}
      <Link to={`/?code=${playgroundCode}`}>Playground</Link>
    </div>
  );
};

export { DocCardProps, DocCard };
