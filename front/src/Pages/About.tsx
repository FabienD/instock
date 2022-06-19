import React from 'react';


// React Components
const About: React.FC = () => {
    
    return (
      <article>
          <h1 className='text-2xl pt-5 pb-10'>
            <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2 mb-1 inline-block" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>
            About
          </h1>
          <section>
            <p>The first intention of this site was to learn new programming languages like Rust or TypeScript, it was a side project.</p>
            <p>Finally, as the project progressed, I thought it could be pushed online. 
              I will continue to learn languages, the project will be improved, I don't know how far it will go.</p>
            <p>The links provided are affiliate links, so if you use it, buy the product, I will be compensated.</p>
          </section>
      </article>
    )
  }

  export default About