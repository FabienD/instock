import React from 'react';
import {Outlet} from 'react-router-dom';
import Footer from './Footer';
import Header from './Header';

const Layout: React.FC = (): JSX.Element => {

  return (
    <>
      <Header />
        <main className='max-w-5xl mr-auto ml-auto px-4 sm:px-6 lg:px-8'>      
          <Outlet />
        </main>
      <Footer />
    </>
  );
};

export default Layout;