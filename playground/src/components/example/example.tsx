/* eslint-disable react/require-default-props */
/* eslint-disable @typescript-eslint/no-empty-function */
import React from 'react';
import AceEditor from 'react-ace';

interface EditorProps {
  children: string | string[];
  height: string;
}

/**
 * Code editor component
 */
const Example = ({ children, height }: EditorProps) => {
  const value = Array.isArray(children) ? children.join('\n') : children;

  return (
    <div style={{ height }}>
      <AceEditor
        className="h-100 font-monospace fs-6"
        focus
        mode="text"
        name="editor"
        readOnly
        showPrintMargin={false}
        highlightActiveLine={false}
        cursorStart={0}
        value={value}
        width="100%"
      />
    </div>
  );
};

export { Example, EditorProps };
