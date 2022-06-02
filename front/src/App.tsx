import React, { useEffect } from 'react';
import './App.css';


const { useState } = React

interface Brand {
  id: string,
  name: string,
  description: string,
}

interface Product {
  id: string,
  name: string,
  description: string,
  brand: Brand,
  url: string,
  upc: string,
  links: TrackingLink[]
}

interface Tracking {
  product_id: string,
  product_name: string,
  links: TrackingLink[],
}

interface TrackingLink {
  merchant_product_url: string,
  merchant: string,
  price: string,
  is_in_stock: boolean,
  tracked_at: string,
}


// Utility and Callback Functions
const getProducts = async (endpoint: string) => {
  const response = await fetch(endpoint)
  const data: Product[] = await response.json()
  return data
}

const getTrackings = async (endpoint: string) => {
  const response = await fetch(endpoint)
  const data: Tracking[] = await response.json()
  return data
}

const generateProductCard = (product: Product, key: number) => {
  const { id, name, description, brand, url, upc, links } = product
  
  return (
    <li key={key} className='card basis-1/2 bg-base-100 shadow-xl'>
      <div className='card-body' data-product-id={id}>
        <h3 className='card-title'>{name}</h3>
        <em>{brand.name} / {upc}</em>
        <div>
          {description}
        </div>
        <Links {...links}/>
      </div>
    </li>
  )
}

const generateTrackingLink = (link: TrackingLink, key: number) => {
  const { merchant_product_url, merchant, price, is_in_stock, tracked_at } = link
  const date = new Date(parseInt(tracked_at) * 1e3)
  const formatedDate = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'medium', timeStyle: 'short' }).format(date)
  
  return (
    <li key={key}>
      <a href={merchant_product_url} className={'inline-block link link-hover'} target="_blank">
        <span className={'badge align-middle ' + (is_in_stock ? 'badge-primary':'badge-error') }>{is_in_stock}</span>
        <h4 className='inline p-1'>{merchant}</h4>
        <em className='p-1'>{price ? ' -> ' + price : ''}</em>
      </a>  
      <p className='ml-6 text-xs italic'>{formatedDate}</p>
    </li>
  )
}

// React Components
const App: React.FC = () => {
  const [products, setProducts] = useState<Product[]>([])
  
  const fetchData = async () => {
    const products = await getProducts(import.meta.env.VITE_API_PRODUCT)
    const trackings = await getTrackings(import.meta.env.VITE_API_TRACKING)

    products!.map((product) => {
      trackings!.map((tracking) => {
        if (product.id == tracking.product_id) {
          product.links = tracking.links
        }
      })
    })

    return setProducts(products)
  }
  
  useEffect(() => {
    fetchData()
  },[]);

  const hasProducts = products.length > 0

  return (
    <div className="">
      { hasProducts ? <Products {...products} /> : null }
    </div>
  )
}

const Products = (items: Product[]) => {
  const products = Object.values(items)
  const hasProducts = products.length > 0
  
  return (
    <ul className="grid grid-cols-2 gap-4">
      {hasProducts ? products!.map(generateProductCard) : null}
    </ul>
  )
}

const Links = (data: TrackingLink[]) => {
  const links = Object.values(data)
  const hasLinks = links.length > 0
  return (
    <ul className="">
      {hasLinks ? links!.map(generateTrackingLink) : null}
    </ul>
  )
    
}

export default App;
