// React Components
import React, { useEffect } from 'react';


const Header: React.FC = () => {

  const navigation = [
    { name: 'Home', href: '/' },
    { name: 'About', href: 'about' },
  ]

    return (
      <>
        <header>
          <nav className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8" aria-label="Top">
            <div className="w-full py-6 flex items-center justify-between border-gray-200 border-b">
              <div className="flex items-center">
                <a href="/">
                  <span className="sr-only">InStock</span>
                  <img
                    className="w-auto"
                    src="/instock_logo.png"
                    alt="InStock?"
                  />
                </a>
                <div className="hidden ml-10 space-x-8 lg:block">
                  {navigation.map((link) => (
                    <a key={link.name} href={link.href} className="text-base font-medium hover:text-indigo-600">
                      {link.name}
                    </a>
                  ))}
                </div>
              </div>
            </div>
            <div className="py-4 flex flex-wrap justify-center space-x-6 lg:hidden">
              {navigation.map((link) => (
                <a key={link.name} href={link.href} className="text-base font-medium hover:text-indigo-600">
                  {link.name}
                </a>
              ))}
            </div>
          </nav>
        </header>
      </>
    )
}

export default Header;