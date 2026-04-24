/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
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
