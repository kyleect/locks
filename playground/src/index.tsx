import './index.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router';
import { createHashRouter } from 'react-router-dom';
import Docs from './pages/docs';
import Playground from './pages/playground';

const router = createHashRouter([
  {
    path: '/*',
    element: <Playground />,
  },
  {
    path: '/docs',
    element: <Docs />,
  },
]);

const mountApp = () => {
  ReactDOM.createRoot(
    document.getElementById('app') as unknown as HTMLElement,
  ).render(
    <React.StrictMode>
      <RouterProvider router={router} />
    </React.StrictMode>,
  );
};
mountApp();
