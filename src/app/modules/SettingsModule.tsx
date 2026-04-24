/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

import React from 'react';
import { ModuleRegistry } from './registry';

export function SettingsModule() {
    return (
        <div className="module-settings glass transition">
            <h2>Settings</h2>
            <div className="settings-content">
                <p>Engine: Tauri (Default)</p>
                <div className="setting-group">
                    <label>Theme: Dark</label>
                </div>
            </div>
        </div>
    );
}

// Register the module
ModuleRegistry.register({
    metadata: {
        id: 'core.settings',
        name: 'Settings',
        version: '0.1.0',
        description: 'Manage BlueCore browser settings',
        icon: '⚙️'
    },
    component: SettingsModule
});
