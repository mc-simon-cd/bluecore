/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

import React, { useState } from 'react';

interface AddressBarProps {
    onNavigate: (url: string) => void;
}

export function AddressBar({ onNavigate }: AddressBarProps) {
    const [url, setUrl] = useState('');

    return (
        <div className="address-bar-container glass">
            <div className="nav-controls">
                <button className="nav-btn transition">←</button>
                <button className="nav-btn transition">→</button>
                <button className="nav-btn transition">↻</button>
            </div>

            <div className="url-input-wrapper transition">
                <input
                    type="text"
                    className="url-input"
                    value={url}
                    onChange={(e) => setUrl(e.target.value)}
                    onKeyDown={(e) => e.key === 'Enter' && onNavigate(url)}
                    placeholder="Search or enter address"
                />
            </div>

            <div className="extra-controls">
                <button className="nav-btn transition">⋮</button>
            </div>
        </div>
    );
}
