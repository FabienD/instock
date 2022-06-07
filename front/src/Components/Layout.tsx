import React from 'react';
import {Outlet} from 'react-router-dom';
import Footer from './Footer';
import Header from './Header';

const Layout: React.FC = (): JSX.Element => {
  return (
    <>
      <Header />
        <main>
        <Outlet />
        </main>
      <Footer />
    </>
  );
};

export default Layout;