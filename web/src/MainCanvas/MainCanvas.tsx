import React, { useEffect, useRef } from 'react'
import "./MainCanvas.css";

const MainCanvas = () => {
    console.log("MainCanvas render");
    
    return (
        <canvas width={500} height={500} id={"main-canvas"}></canvas>
    );
}

export default MainCanvas;