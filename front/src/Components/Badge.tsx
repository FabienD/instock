import React from 'react';

interface BadgeProps {
    is_in_stock: boolean,
}

const Badge: React.FC<BadgeProps> = (props: BadgeProps) => {

    const badgeSvg = props.is_in_stock ? 'M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z' : 'M6 18L18 6M6 6l12 12'
    const badgeBgColor = props.is_in_stock ? 'bg-green-600' : 'bg-rose-700'
    const badgeTextColor = props.is_in_stock ? 'text-green-100' : 'text-rose-100'
    const color = 'inline-flex items-center px-1 py-1 rounded-full text-xs font-medium ' + badgeBgColor + ' ' + badgeTextColor

    return (
        <span className={color} >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
                <path strokeLinecap="round" strokeLinejoin="round" d={badgeSvg} />
            </svg>    
        </span>
     )
}

export default Badge