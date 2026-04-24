/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
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
