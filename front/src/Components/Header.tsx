// React Components
import React, { useEffect } from 'react';

const Header: React.FC = () => {

    return (
       <div className="hero bg-base-100">
        <div className="hero-content text-center text-neutral-content-focus">
          <div className="max-w-md">
            
            <h1 className="mb-5 text-5xl font-bold">
              <span className='text-primary'>In</span>Stock?
            </h1>
            <p className="mb-5">
              For months, some game consoles have been out of stock. <br />
              This small project tries to give an overview of the availability of these products.
            </p>
          </div>
        </div>
      </div>
    )
}

export default Header;