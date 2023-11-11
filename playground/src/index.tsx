import './index.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router';
import { createHashRouter } from 'react-router-dom';
import { ScrollToAnchor } from './components/scroll-to-anchor/ScrollToAnchor';
import Docs from './pages/docs';
import Playground from './pages/playground';
import Tooling from './pages/tooling';

const router = createHashRouter([
  {
    path: '/*',
    element: <Playground />,
  },
  {
    path: '/docs',
    element: (
      <>
        <ScrollToAnchor />
        <Docs />
      </>
    ),
  },
  {
    path: '/tooling',
    element: <Tooling />,
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
