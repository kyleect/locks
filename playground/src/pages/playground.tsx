/* eslint-disable no-restricted-globals */
import React, { useEffect, useRef, useState } from 'react';
import { Popover } from 'bootstrap';
import { useSearchParams, useNavigate } from 'react-router-dom';
import Split from 'react-split';
import {
  compressToEncodedURIComponent,
  decompressFromEncodedURIComponent,
  // eslint-disable-next-line import/no-relative-packages
} from '../../vendor/lz-string';

import { Editor } from '../components/editor';
import { LocksDisassembleButton } from '../components/locks-disassemble-button';
import { LocksRunButton } from '../components/locks-run-button';
import { Navbar } from '../components/navbar';
import { Output } from '../components/output';
import { useLocks } from '../hooks/useLocks';

class LocalStorage {
  static editorTextKey = 'editorText';

  static get editorText(): string | null {
    return localStorage.getItem(this.editorTextKey);
  }

  static set editorText(text: string) {
    localStorage.setItem(this.editorTextKey, text);
  }
}

/**
 * Locks's playground page component
 * @returns A page component
 */
const Playground: React.FC = () => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const { isRunning, runLocks, disassembleLocks, stopLocks, locksResult } =
    useLocks();

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
    if (editorText?.length === 0) {
      return;
    }

    const compressedText = compressToEncodedURIComponent(editorText);

    const hash = `#/?code=${compressedText}`;

    if (history.replaceState) {
      history.replaceState(null, '', hash);
    } else {
      location.hash = hash;
    }

    LocalStorage.editorText = compressedText;
  }, [editorText, navigate]);

  useEffect(() => {
    if (searchParams.get('code')) {
      const code = searchParams.get('code')?.trim() ?? '';
      let userCode = decompressFromEncodedURIComponent(code);
      // Fallback incase there is an extra level of decoding:
      // https://gitter.im/Microsoft/TypeScript?at=5dc478ab9c39821509ff189a
      if (!userCode)
        userCode = decompressFromEncodedURIComponent(decodeURIComponent(code));
      setEditorText(userCode);
    } else if (LocalStorage.editorText) {
      const decompressedCode = decompressFromEncodedURIComponent(
        LocalStorage.editorText,
      );
      setEditorText(decompressedCode);
    }
  }, [searchParams]);
  /**
   * @remarks
   * Send resize signal to editor on split resize.
   * Refer to {@link https://github.com/securingsincity/react-ace/issues/708}
   */
  const resizeHandler = () => window.dispatchEvent(new Event('resize'));

  const clipboardRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    // eslint-disable-next-line no-new, @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-argument
    new Popover(clipboardRef.current as any, {
      content: 'Copied!',
      trigger: 'focus',
    });
  }, []);

  return (
    <>
      <Navbar
        subBrandText="Playground"
        content={
          <>
            <button
              className="btn btn-primary me-1"
              type="button"
              // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
              ref={clipboardRef}
              onClick={() => {
                navigator.clipboard
                  .writeText(window.location.href)
                  .catch(() => {
                    // eslint-disable-next-line no-alert
                    alert('Unable to copy playground link');
                  });
              }}
              aria-label="Github repository"
            >
              <span
                className="bi bi-clipboard"
                role="status"
                aria-hidden="true"
              />
            </button>
            <LocksDisassembleButton
              isRunning={isRunning}
              onClick={
                isRunning ? stopLocks : () => disassembleLocks(editorText)
              }
            />
            <LocksRunButton
              isRunning={isRunning}
              onClick={isRunning ? stopLocks : () => runLocks(editorText)}
            />
          </>
        }
      />
      <Split
        className="d-flex"
        cursor="col-resize"
        direction="horizontal"
        id="content"
        onDragEnd={resizeHandler}
      >
        <Editor text={editorText} onChange={setEditorText} />
        <Output text={locksResult ?? ''} />
      </Split>
    </>
  );
};

export default Playground;
