import React, { useEffect } from 'react';
import './App.css';


const { useState } = React


interface TrackedProduct {
  product_name:     string;
  product_url:      string;
  product_merchant: string;
  is_in_stock:      boolean;
  tracked_at:       number;
}

// Utility and Callback Functions
const getData = async (endpoint: string) => {
  const response = await fetch(endpoint)
  const data: TrackedProduct[] = await response.json()
  return data
}

const generateProductLink = (product: TrackedProduct, key: number) => {
  const { product_name, product_url, is_in_stock, tracked_at } = product
  const bg_name: string = is_in_stock ? 'px-4 py-2 border-l-4 border-l-green-500 my-2': 'px-4 py-2 border-l-4 border-l-red-500 my-2'
  const date: Date = new Date(tracked_at * 1000)

  return (
    <li key={key} className={bg_name}>
        <a href={product_url} target="_blank">
          <span>{product_name}</span>
        </a>
        <br />
        <em className='text-xs'>{date.toLocaleDateString('fr-FR')} à {date.toLocaleTimeString('fr-FR')}</em>
    </li>
  )
}

// React Components
const App: React.FC = () => {
  const [items, setItems] = useState<TrackedProduct[]>([])
  
  const fetchProducts = async () => {
     const productsData = await getData('http://127.0.0.1:8080/api/products/last')
     return setItems(productsData)
  }

  useEffect(() => {
    fetchProducts()
  },[]);

  const hasProducts = items.length > 0

  return (
    <div className='max-w-md w-full space-y-8'>
      <h1 className='text-xl font-medium'>Which game console is available ?</h1>
      <div>
        { hasProducts ? <Products {...items} /> : null }
      </div>
    </div>
  )
}

const Products = (items: TrackedProduct[]) => {
  const products = Object.values(items)
  const hasProducts = products.length > 0
  
  return (
    <ul className="list-outside">
      {hasProducts ? products!.map(generateProductLink) : null}
    </ul>
  )
}

export default App;
