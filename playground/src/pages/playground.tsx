/* eslint-disable no-restricted-globals */
import React, { useEffect, useState } from 'react';
import Split from 'react-split';
import {
  compressToEncodedURIComponent,
  decompressFromEncodedURIComponent,
  // eslint-disable-next-line import/no-relative-packages
} from '../../vendor/lz-string';

import { Editor } from '../components/editor';
import { Navbar } from '../components/navbar';
import { Output } from '../components/output';

class LocalStorage {
  static editorTextKey = 'editorText';

  static get editorText(): string | null {
    return localStorage.getItem(this.editorTextKey);
  }

  static set editorText(text: string) {
    localStorage.setItem(this.editorTextKey, text);
  }
}

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

type LoxOutMessage =
  | LoxOutMessageOutput
  | LoxOutMessageExitFailure
  | LoxOutMessageExitSuccess;

/**
 * Locks's playground page component
 * @returns A page component
 */
const Playground: React.FC = () => {
  /**
   * @remarks
   * Sends resize signal to editor on initialization.
   * Refer to {@link https://github.com/securingsincity/react-ace/issues/70}
   */
  useEffect(() => {
    window.dispatchEvent(new Event('resize'));
  }, []);

  // Editor text is saved to local storage.
  const [editorText, setEditorText] = useState<string>('');

  useEffect(() => {
    if (editorText.length === 0) {
      return;
    }

    const hash = `#/code=${compressToEncodedURIComponent(editorText)}`;

    if (history.replaceState) {
      history.replaceState(null, '', hash);
    } else {
      location.hash = hash;
    }

    LocalStorage.editorText = editorText;
  }, [editorText]);

  useEffect(() => {
    if (location.hash.startsWith('#/code')) {
      const code = location.hash.replace('#/code=', '').trim();
      let userCode = decompressFromEncodedURIComponent(code);
      // Fallback incase there is an extra level of decoding:
      // https://gitter.im/Microsoft/TypeScript?at=5dc478ab9c39821509ff189a
      if (!userCode)
        userCode = decompressFromEncodedURIComponent(decodeURIComponent(code));
      setEditorText(userCode);
    } else if (LocalStorage.editorText) {
      setEditorText(LocalStorage.editorText);
    }
  }, []);

  // Output from Lox is continuously streamed here.
  const [outputText, setOutputText] = useState<string>('');
  const addOutputText = (text: string) => {
    setOutputText((currentOutput) => currentOutput + text);
  };

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

  const startLox = () => {
    stopWorker();
    setOutputText('');

    const webWorker = new Worker(new URL('../worker.ts', import.meta.url), {
      type: 'module',
    });

    webWorker.onmessage = (event) => {
      const msg: LoxOutMessage = JSON.parse(
        event.data as string,
      ) as LoxOutMessage;

      switch (msg.type) {
        case 'Output':
          addOutputText(msg.text);
          break;
        case 'ExitSuccess':
          stopWorker();
          addOutputText('---\nProgram exited successfully.\n');
          break;
        case 'ExitFailure':
          stopWorker();
          addOutputText('---\nProgram exited with errors.\n');
          break;
        default:
          break;
      }
    };

    webWorker.postMessage(editorText);
    setWorker(webWorker);
  };

  const stopLox = () => {
    stopWorker();
    addOutputText('---\nCommand terminated.');
  };

  const isRunning = worker !== null;
  /**
   * @remarks
   * Send resize signal to editor on split resize.
   * Refer to {@link https://github.com/securingsincity/react-ace/issues/708}
   */
  const resizeHandler = () => window.dispatchEvent(new Event('resize'));

  return (
    <>
      <Navbar
        isRunning={isRunning}
        onRunClick={isRunning ? stopLox : startLox}
      />
      <Split
        className="d-flex"
        cursor="col-resize"
        direction="horizontal"
        id="content"
        onDragEnd={resizeHandler}
      >
        <Editor text={editorText} onChange={setEditorText} />
        <Output text={outputText} />
      </Split>
    </>
  );
};

export default Playground;
