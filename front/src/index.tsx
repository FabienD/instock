import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';


const entryPoint = document.querySelector('#root')

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  entryPoint
);