/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

import React from 'react';
import { useModules } from '../../hooks/useModules';

export function ModuleCenter() {
    const { modules, loading, toggleModule } = useModules();

    if (loading) {
        return <div className="p-8 text-text-secondary animate-pulse">Loading modules...</div>;
    }

    return (
        <div className="p-6 bg-bg-secondary/50 backdrop-blur-glass min-h-full text-text-primary">
            <header className="mb-8">
                <h1 className="text-2xl font-bold mb-2">Module Center</h1>
                <p className="text-text-secondary">Govern BlueCore features and privacy protections.</p>
            </header>

            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-1">
                {modules.map((module) => (
                    <div
                        key={module.metadata.id}
                        className="p-4 rounded-xl border border-white/5 bg-white/5 hover:bg-white/10 transition-all duration-300 group"
                    >
                        <div className="flex items-center justify-between mb-2">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-lg bg-accent-primary/20 flex items-center justify-center text-accent-primary text-xl">
                                    {module.metadata.icon || '🧩'}
                                </div>
                                <div>
                                    <h3 className="font-semibold">{module.metadata.name}</h3>
                                    <span className="text-xs text-text-muted">v{module.metadata.version}</span>
                                </div>
                            </div>

                            <label className="relative inline-flex items-center cursor-pointer">
                                <input
                                    type="checkbox"
                                    className="sr-only peer"
                                    checked={module.isActive}
                                    onChange={(e) => toggleModule(module.metadata.id, e.target.checked)}
                                />
                                <div className="w-11 h-6 bg-white/10 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent-primary"></div>
                            </label>
                        </div>

                        <p className="text-sm text-text-secondary mb-4 leading-relaxed">
                            {module.metadata.description}
                        </p>

                        <div className="flex flex-wrap gap-2">
                            {module.metadata.permissions.map(perm => (
                                <span key={perm} className="px-2 py-1 bg-white/5 rounded text-[10px] uppercase tracking-wider text-text-muted">
                                    {perm}
                                </span>
                            ))}
                        </div>
                    </div>
                ))}
            </div>

            {modules.length === 0 && (
                <div className="text-center py-12 text-text-muted italic">
                    No modules registered in the system.
                </div>
            )}
        </div>
    );
}
