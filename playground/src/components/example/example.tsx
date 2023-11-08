/* eslint-disable @typescript-eslint/no-empty-function */
import React from 'react';
import AceEditor from 'react-ace';

interface EditorProps {
  children: string | string[];
}

/**
 * Code editor component
 */
const Example = ({ children }: EditorProps) => {
  const lines = Array.isArray(children)
    ? children.length
    : (children ?? '').split('\n').length;

  const value = Array.isArray(children) ? children.join('\n') : children;

  return (
    <AceEditor
      className="h-100 font-monospace fs-6"
      focus
      mode="text"
      name="editor"
      readOnly
      showPrintMargin={false}
      maxLines={lines}
      value={value}
    />
  );
};

export { Example, EditorProps };
