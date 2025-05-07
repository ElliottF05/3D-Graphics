import React, { useEffect, useRef } from 'react'

const MainCanvas = () => {
    console.log("MainCanvas render");
    // The parent div in App.tsx (flex items-center justify-center p-4) will center this canvas.
    // aspect-square ensures it's a square.
    // max-w-full and max-h-full make it fit the padded container from App.tsx.
    return (
        <canvas 
            width={500} // Internal drawing resolution width
            height={500} // Internal drawing resolution height
            id={"main-canvas"} 
            className="block bg-neutral-800 aspect-square max-w-full max-h-full shadow-lg" // Added shadow for better visual separation
        ></canvas>
    );
}

export default MainCanvas;