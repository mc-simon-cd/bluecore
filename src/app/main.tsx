/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 * Part of the Simon Project BlueCore Browser
 */

import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserShell } from './layout/BrowserShell';
import '../index.css';

export function App() {
    return (
        <div className="app-container">
            <BrowserShell />
        </div>
    );
}

const root = document.getElementById('root');
if (root) {
    ReactDOM.createRoot(root).render(
        <React.StrictMode>
            <App />
        </React.StrictMode>
    );
}
