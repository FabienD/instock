import React, { useEffect } from 'react';


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
      <div className='bg-base-300 shadow-xl'>
        <figure className='bg-white'>
          <img src={image} alt={name} className='p-5' />
        </figure>
        <div className='' data-product-id={id}>
          <h3 className=''>{name}</h3>
          <p>{description}</p>
          <div className=''></div> 
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
const Home: React.FC = () => {
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
    <article>
      <h1 className='text-2xl pt-5 pb-10'>
      <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2 mb-1 inline-block" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
      </svg>
      Game consoles availability
      </h1>
      { hasProducts ? <Products {...products} /> : null }
    </article>
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
    <>
      {hasLinks ? links!.map(generateTrackingLink) : null}
    </>
  )
    
}

export default Home;
