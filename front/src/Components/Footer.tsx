// React Components
import React from 'react';
import {Link} from 'react-router-dom';

const Footer: React.FC = () => {

    return (
        <footer className='footer footer-center p-10 text-base-content'>
          <div className="grid grid-flow-col gap-4">
            <a className="link link-hover"><Link to='/'>Home</Link></a> 
            <a className="link link-hover"><Link to='about'>About</Link></a> 
          </div>
          <div className='grid grid-flow-col gap-4'>
            <p>
              <a href="https://github.com/FabienD/instock" className='link link-hover link-pr' target="_blank">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 inline" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
              </svg>
              <span className='pl-1'>An open source side project</span> 
            </a>
            </p>
          </div>
      </footer>
    )
}

export default Footer;
