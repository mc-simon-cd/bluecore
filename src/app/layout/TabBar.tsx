/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

import React from 'react';

interface Tab {
    id: string;
    title: string;
    active: boolean;
}

export function TabBar() {
    const tabs: Tab[] = [
        { id: '1', title: 'New Tab', active: true }
    ];

    return (
        <div className="tab-bar-container glass">
            {tabs.map((tab) => (
                <div
                    key={tab.id}
                    className={`tab-item transition ${tab.active ? 'active' : ''}`}
                >
                    <span className="tab-title">{tab.title}</span>
                    <button className="tab-close">×</button>
                </div>
            ))}
            <button className="new-tab-btn transition">+</button>
        </div>
    );
}
