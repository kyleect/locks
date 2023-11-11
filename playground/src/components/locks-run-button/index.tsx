/* eslint-disable @typescript-eslint/restrict-template-expressions */
import React from 'react';

interface LocksRunButtonProp {
  isRunning: boolean;
  onClick: () => void;
  className?: string;
}

const LocksRunButton: React.FC<LocksRunButtonProp> = ({
  isRunning,
  onClick,
  className,
}) => {
  let runColor = 'btn-success';
  let runIcon = 'me-1 bi bi-play-fill';
  let runText = 'Run';

  if (isRunning) {
    runColor = 'btn-danger';
    runIcon = 'me-2 spinner-grow spinner-grow-sm';
    runText = 'Stop';
  }

  return (
    <button
      id="run-btn"
      className={`btn ${runColor} ${className}`}
      onClick={onClick}
      type="button"
      aria-label="Run code"
    >
      <span className={runIcon} role="status" aria-hidden="true" />
      {runText}
    </button>
  );
};

LocksRunButton.defaultProps = {
  className: '',
};

export { LocksRunButtonProp, LocksRunButton };
