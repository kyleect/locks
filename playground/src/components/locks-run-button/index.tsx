import React from 'react';

interface LocksRunButtonProp {
  isRunning: boolean;
  onClick: () => void;
}

const LocksRunButton: React.FC<LocksRunButtonProp> = ({
  isRunning,
  onClick,
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
      className={`btn ${runColor}`}
      onClick={onClick}
      type="button"
      aria-label="Run code"
    >
      <span className={runIcon} role="status" aria-hidden="true" />
      {runText}
    </button>
  );
};

export { LocksRunButtonProp, LocksRunButton };
