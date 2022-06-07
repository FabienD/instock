import { createRoot } from 'react-dom/client';
import App from './Components/App';
import {
  BrowserRouter
} from "react-router-dom";

import './App.css';

const container = document.getElementById('root')
const root = createRoot(container!)

root.render(
  <BrowserRouter>
    <App />
  </BrowserRouter>
)
