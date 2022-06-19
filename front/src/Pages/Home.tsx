import React, { useEffect } from 'react';
import Badge from '../Components/Badge';

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


// Call API products + tracking
const getData = async () => {
  const urls: string[] = [
    import.meta.env.VITE_API_PRODUCT,
    import.meta.env.VITE_API_TRACKING,
  ]
  
  return Promise.all(urls.map(url => fetch(url)))
    .then(responses => {
      return Promise.all(responses.map(response => response.json()))
    })
    .catch(error => {
        console.error(error)
    })
}

const generateProductCard = (product: Product, key: number) => {
  const { id, name, description, image, links } = product
  
  return (
    <li key={key} className='bg-sky-50 rounded-xl shadow-md'>
      <figure className='bg-white text-center p-5'>
        <img src={image} alt={name} className='inline' />
      </figure>
      <div className='p-4' data-product-id={id}>
        <h3 className='font-bold pb-4'>{name}</h3>
        <p className='pb-4'>{description}</p>
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
    <li key={key} className='p-2'>
      <a href={merchant_product_url} className='grid grid-cols-2' target="_blank">
        <div className=''>
          <Badge is_in_stock={is_in_stock} />
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
    const [products, trackings] = await getData()
    
    products!.map((product: Product) => {
      trackings!.map((tracking: Tracking) => {
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
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
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
    <ul className='grid sm:grid-cols-1 sm:gap-5 md:grid-cols-2 md:gap-10'>
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

export default Home;
