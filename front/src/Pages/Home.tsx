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
    // @ts-ignore
    import.meta.env.VITE_API_PRODUCT,
    // @ts-ignore
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
    <li key={key} className='bg-gradient-to-r from-indigo-100 to-indigo-50 rounded-xl shadow-md'>
      <figure className='text-center bg-white rounded-t-xl p-5 border border-indigo-50'>
        <img src={image} alt={name} width="200" height="169" className='inline' />
      </figure>
      <div className='p-4 text-gray-500' data-product-id={id}>
        <h3 className='font-bold pb-4 text-indigo-800'>{name}</h3>
        <p className='pb-4'>{description}</p>
        <Links {...links}/>
      </div>
    </li>
  )
}

const generateTrackingLink = (link: TrackingLink, key: number) => {

  const { merchant_product_url, merchant, price, is_in_stock, tracked_at } = link
  const date = new Date(parseInt(tracked_at) * 1e3)
  const formatedDate = new Intl.DateTimeFormat('en-US', { dateStyle: 'short', timeStyle: 'short' }).format(date)
  
  return (
    <li key={key} className='p-2'>
      <a href={merchant_product_url} className='grid grid-cols-2' target="_blank">
        <div className=''>
          <Badge is_in_stock={is_in_stock} />
          <h4 className='inline p-1 ml-2'>{merchant}</h4>
          <em className='font-bold text-xs'>{price ? ' -> ' + price : ''}</em>
        </div>
        <span className='text-xs text-right italic leading-6'>{formatedDate}</span>
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
    <>
      <div className="bg-white">
        <div className="max-w-5xl mx-auto py-16 px-4 sm:py-24 sm:px-6 lg:px-8">
          <div className="text-center">
            <h1 className="text-base font-semibold text-indigo-600 tracking-wide uppercase">In Stock?</h1>
            <p className="mt-1 text-4xl font-extrabold text-gray-900 sm:text-5xl sm:tracking-tight lg:text-6xl">
              Last tracking.
            </p>
          </div>
        </div>
      </div>
      <article>
        { hasProducts ? <Products {...products} /> : null }
      </article>
    </>
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
