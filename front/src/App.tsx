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
  image: string,
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
  const { id, name, description, image, links } = product
  
  return (
    <li key={key} className='xl:w-1/2 md:w-1/1 p-5'>
      <div className='card bg-base-300 shadow-xl'>
        <figure className='bg-white'>
          <img src={image} alt={name} className='p-5' />
        </figure>
        <div className='card-body' data-product-id={id}>
          <h3 className='card-title'>{name}</h3>
          <p>{description}</p>
          <div className='divider'></div> 
          <Links {...links}/>
        </div>
      </div>
    </li>
  )
}

const generateTrackingLink = (link: TrackingLink, key: number) => {
  const { merchant_product_url, merchant, price, is_in_stock, tracked_at } = link
  const date = new Date(parseInt(tracked_at) * 1e3)
  const formatedDate = new Intl.DateTimeFormat('fr-FR', { dateStyle: 'medium', timeStyle: 'short' }).format(date)
  
  return (
    <li key={key} className='p-2'>
      <a href={merchant_product_url} className={'inline-block link link-hover grid grid-cols-2'} target="_blank">
        <div className=''>
          <span className={'badge align-middle ' + (is_in_stock ? 'badge-primary':'badge-error') }>{is_in_stock}</span>
          <h4 className='inline p-1 ml-2'>{merchant}</h4>
          <em className='p-1'>{price ? ' -> ' + price : ''}</em>
        </div>
        <span className='text-xs text-right italic'>{formatedDate}</span>
      </a>
    </li>
  )
}

// React Components
const App: React.FC = () => {
  const [products, setProducts] = useState<Product[]>([])
  
  const fetchData = async () => {
    // @ts-ignore
    const products = await getProducts(import.meta.env.VITE_API_PRODUCT)
    // @ts-ignore
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
    <ul className='flex flex-wrap items-stretch -m-5'>
      {hasProducts ? products!.map(generateProductCard) : null}
    </ul>
  )
}

const Links = (data: TrackingLink[]) => {
  const links = Object.values(data)
  const hasLinks = links.length > 0
  return (
    <ul className=''>
      {hasLinks ? links!.map(generateTrackingLink) : null}
    </ul>
  )
    
}

export default App;
