/* eslint-disable @typescript-eslint/restrict-template-expressions */
import React from 'react';

interface LocksParseButtonProps {
  isRunning: boolean;
  onClick: () => void;
  className?: string;
}

const LocksParseButton: React.FC<LocksParseButtonProps> = ({
  isRunning,
  onClick,
  className,
}) => {
  let runColor = 'btn-secondary';
  let runIcon = 'me-1 bi bi-file-earmark-code-fill';
  let runText = 'Parse';

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
      <span className="d-none d-lg-inline">{runText}</span>
    </button>
  );
};

LocksParseButton.defaultProps = {
  className: '',
};

export { LocksParseButtonProps, LocksParseButton };
