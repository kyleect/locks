import './index.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import Playground from './pages/playground';

const mountApp = () => {
  ReactDOM.createRoot(
    document.getElementById('app') as unknown as HTMLElement,
  ).render(
    <React.StrictMode>
      <Playground />
    </React.StrictMode>,
  );
};
mountApp();
