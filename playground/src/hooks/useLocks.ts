import { useState } from 'react';

type LoxOutMessageOutput = {
  type: 'Output';
  text: string;
};

type LoxOutMessageExitFailure = {
  type: 'ExitFailure';
};

type LoxOutMessageExitSuccess = {
  type: 'ExitSuccess';
};

type LocksDiagnosisPosition = {
  line: number;
  character: number;
};

type LocksDiagnosisRange = {
  start: LocksDiagnosisPosition;
  end: LocksDiagnosisPosition;
};

enum LocksDiagnosisSeverity {
  ERROR = 1,
  WARNING = 2,
  INFORMATION = 3,
  HINT = 4,
}

type LocksDiagnosis = {
  range: LocksDiagnosisRange;
  severity?: LocksDiagnosisSeverity;
  message: string;
};

type LoxOutMessageDiagnostics = {
  type: 'Diagnostics';
  diagnostics: LocksDiagnosis[];
};

type LoxOutMessage =
  | LoxOutMessageOutput
  | LoxOutMessageExitFailure
  | LoxOutMessageExitSuccess
  | LoxOutMessageDiagnostics;

// eslint-disable-next-line import/prefer-default-export
export function useLocks() {
  // Output from Lox is continuously streamed here.
  const [locksResult, setLocksResult] = useState<string | null>(null);
  const appendToLocksResult = (text: string) => {
    setLocksResult((currentOutput) => (currentOutput ?? '') + text);
  };
  const [diagnostics, setDiagnostics] = useState<LocksDiagnosis[]>([]);

  // The worker is set back to null once it finishes executing.
  const [worker, setWorker] = useState<Worker | null>(null);
  const stopWorker = () => {
    setWorker((currentWorker) => {
      if (currentWorker !== null) {
        currentWorker.terminate();
      }
      return null;
    });
  };

  const runLocks = (code: string) => {
    stopWorker();
    setLocksResult(null);

    const webWorker = new Worker(new URL('../worker.ts', import.meta.url), {
      type: 'module',
    });

    webWorker.onmessage = (event) => {
      const msg: LoxOutMessage = JSON.parse(
        event.data as string,
      ) as LoxOutMessage;

      switch (msg.type) {
        case 'Output':
          appendToLocksResult(msg.text);
          break;
        case 'Diagnostics':
          setDiagnostics(msg.diagnostics);
          break;
        case 'ExitSuccess':
          stopWorker();
          break;
        case 'ExitFailure':
          stopWorker();
          break;
        default:
          break;
      }
    };

    webWorker.postMessage({ code, action: 'run' });
    setWorker(webWorker);
  };

  const disassembleLocks = (code: string) => {
    stopWorker();
    setLocksResult(null);

    const webWorker = new Worker(new URL('../worker.ts', import.meta.url), {
      type: 'module',
    });

    webWorker.onmessage = (event) => {
      const msg: LoxOutMessage = JSON.parse(
        event.data as string,
      ) as LoxOutMessage;

      switch (msg.type) {
        case 'Output':
          appendToLocksResult(msg.text);
          break;
        case 'Diagnostics':
          setDiagnostics(msg.diagnostics);
          break;
        case 'ExitSuccess':
          stopWorker();
          break;
        case 'ExitFailure':
          stopWorker();
          break;
        default:
          break;
      }
    };

    webWorker.postMessage({ code, action: 'disassemble' });
    setWorker(webWorker);
  };

  const diagnoseLocks = (code: string) => {
    stopWorker();
    setLocksResult(null);

    const webWorker = new Worker(new URL('../worker.ts', import.meta.url), {
      type: 'module',
    });

    webWorker.onmessage = (event) => {
      const msg: LoxOutMessage = JSON.parse(
        event.data as string,
      ) as LoxOutMessage;

      switch (msg.type) {
        case 'Output':
          appendToLocksResult(msg.text);
          break;
        case 'Diagnostics':
          setDiagnostics(msg.diagnostics);
          break;
        case 'ExitSuccess':
          stopWorker();
          break;
        case 'ExitFailure':
          stopWorker();
          break;
        default:
          break;
      }
    };

    webWorker.postMessage({ code, action: 'diagnose' });
    setWorker(webWorker);
  };

  const stopLocks = () => {
    stopWorker();
  };

  const isRunning = worker !== null;

  return {
    isRunning,
    locksResult,
    runLocks,
    disassembleLocks,
    stopLocks,
    diagnostics,
    diagnoseLocks,
  };
}
