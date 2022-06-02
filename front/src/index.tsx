import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import Header from './Components/Header';
import Footer from './Components/Footer';


const entryPoint = document.querySelector('#root')

ReactDOM.render(
  <React.StrictMode>
    <Header />
      <App />
    <Footer />
  </React.StrictMode>,
  entryPoint
);