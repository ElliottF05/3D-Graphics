import React from 'react';
import ReactDOM from 'react-dom/client';
import * as wasm from '../wasm/wasm_graphics'

import App from './App.tsx'

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);

await wasm.default();

wasm.init_and_begin_game_loop();