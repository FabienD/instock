import React from 'react';
import {Navigate, useRoutes} from 'react-router-dom';

import Layout from './Layout';
import About from '../Pages/About';
import Home from '../Pages/Home';
import PageNotFound from '../Pages/PageNotFound';

const App: React.FC = (): JSX.Element => {
  const mainRoutes = {
    path: '/',
    element: <Layout />,
    children: [
      {path: '*', element: <Navigate to='/404' />},
      {path: '/', element: <Home />},
      {path: '404', element: <PageNotFound />},
      {path: 'about', element: <About />},
    ],
  };

  const routing = useRoutes([mainRoutes]);

  return <>{routing}</>;
};

export default App;