/* eslint-disable @typescript-eslint/restrict-template-expressions */
import React from 'react';

interface LocksDisassembleButtonProp {
  isRunning: boolean;
  onClick: () => void;
  className?: string;
}

const LocksDisassembleButton: React.FC<LocksDisassembleButtonProp> = ({
  isRunning,
  onClick,
  className,
}) => {
  let runColor = 'btn-secondary';
  let runIcon = 'me-1 bi bi-columns-gap';
  let runText = 'Disassemble';

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

LocksDisassembleButton.defaultProps = {
  className: '',
};

export { LocksDisassembleButtonProp, LocksDisassembleButton };
