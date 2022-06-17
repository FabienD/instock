// React Components
import React, { useEffect } from 'react';

const Header: React.FC = () => {

    return (
       <header>
          <div className="text-center p-10">
            <h1 className="mb-5 text-5xl font-bold">
              <span className='text-sky-600'>In</span>Stock?
            </h1>
            <p className="mb-5">
              For months, some game consoles have been out of stock. <br />
              This small project tries to give an overview of the availability of these products.
            </p>
          </div>
      </header>
    )
}

export default Header;