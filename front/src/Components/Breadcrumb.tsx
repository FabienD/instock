import React from 'react';
import useBreadcrumbs from 'use-react-router-breadcrumbs';
import { ChevronRightIcon, HomeIcon } from '@heroicons/react/solid'
import { NavLink } from 'react-router-dom';

const Breadcrumbs: React.FC = (): JSX.Element => {
  
  const breadcrumbs = useBreadcrumbs()

  return (
    <nav className="flex max-w-5xl mr-auto ml-auto" aria-label="Breadcrumb">
        <ol role="list" className="flex items-center space-x-4">
            <li>
            <div>
                <a href="#" className="text-gray-400 hover:text-gray-500">
                <HomeIcon className="flex-shrink-0 h-5 w-5" aria-hidden="true" />
                <span className="sr-only">Home</span>
                </a>
            </div>
            </li>
            {breadcrumbs.map(({
                match,
                breadcrumb
            }) => (
                <li key={match.pathname}>
                    <div className="flex items-center">
                        <ChevronRightIcon className="flex-shrink-0 h-5 w-5 text-gray-400" aria-hidden="true" />
                        <NavLink to={match.pathname} className="ml-4 text-sm font-medium text-gray-500 hover:text-gray-700">{breadcrumb}</NavLink>
                    </div>
                </li>
            ))}
        </ol>
    </nav>
  )
}

export default Breadcrumbs