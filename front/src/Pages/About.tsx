import React from 'react';


// React Components
const About: React.FC = () => {
    
    return (
      <>
        <div className="bg-white">
          <div className="max-w-5xl mx-auto py-16 px-4 sm:py-24 sm:px-6 lg:px-8">
            <div className="text-center">
              <h1 className="text-base font-semibold text-indigo-600 tracking-wide uppercase">About</h1>
              <p className="mt-1 text-4xl font-extrabold text-gray-900 sm:text-5xl sm:tracking-tight lg:text-6xl">
                A little side project.
              </p>
            </div>
          </div>
        </div>
        <article>
            <section className='prose text-center max-w-2xl mx-auto'>
              <p>The first intention of this site was to learn new programming languages like
              &nbsp;<a href="https://www.rust-lang.org/" target='_blank' className="underline decoration-2 decoration-indigo-500">Rust</a> or 
              &nbsp;<a href="https://www.typescriptlang.org/" target='_blank' className="underline decoration-2 decoration-indigo-500">TypeScript</a>, it was a side project.
              </p>
              <p>Finally, as the project progressed, I thought it could be pushed online. 
                I will continue to learn languages, the project will be improved, I don't know how far it will go.</p>
              <p>The links provided are affiliate links, so if you use it, buy the product, I will be compensated.</p>
            </section>
        </article>
      </>
    )
  }

  export default About