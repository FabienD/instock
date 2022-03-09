import React, { useEffect } from 'react';
import './App.css';


const { useState } = React

interface Brand {
  id: BigInteger,
  name: string,
  description: string,
}

interface Product {
  id: BigInteger,
  name: string,
  description: string,
  brand: Brand,
  url: string,
  upc: string,
}

// Utility and Callback Functions
const getData = async (endpoint: string) => {
  const response = await fetch(endpoint)
  const data: Product[] = await response.json()
  return data
}

const generateProductLink = (product: Product, key: number) => {
  const { id, name, description, brand, url, upc } = product
  
  return (
    <li key={key} className='card basis-1/2 bg-base-100 shadow-xl'>
      <div className='card-body'>
        <h3 className='card-title'>{name}</h3>
        <em>{brand.name} / {upc}</em>
        <div>
          {description}
        </div>
      </div>
    </li>
  )
}

// React Components
const App: React.FC = () => {
  const [items, setItems] = useState<Product[]>([])
  
  const fetchProducts = async () => {
     const productsData = await getData('http://127.0.0.1:8080/api/product/all')
     return setItems(productsData)
  }

  useEffect(() => {
    fetchProducts()
  },[]);

  const hasProducts = items.length > 0

  return (
    <div className=''>
      <h1 className=''>Which game console is available ?</h1>
      { hasProducts ? <Products {...items} /> : null }
    </div>
  )
}

const Products = (items: Product[]) => {
  const products = Object.values(items)
  const hasProducts = products.length > 0
  
  return (
    <ul className="grid grid-cols-2 gap-4">
      {hasProducts ? products!.map(generateProductLink) : null}
    </ul>
  )
}

export default App;
