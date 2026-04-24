/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { TabBar } from './TabBar';
import { AddressBar } from './AddressBar';
import { useModules } from '../../hooks/useModules';
import { ModuleRegistry } from '../modules/registry';
import { ModuleCenter } from '../../components/Settings/ModuleCenter';

export function BrowserShell() {
    const { modules, toggleModule } = useModules();
    const [currentView, setCurrentView] = React.useState<'browser' | 'settings'>('browser');

    const handleNavigate = async (url: string) => {
        if (url === 'bluecore://settings') {
            setCurrentView('settings');
            return;
        }
        setCurrentView('browser');
        await invoke('create_tab', { url });
    };

    return (
        <div id="bluecore-shell" className="flex flex-col h-screen bg-bg-primary overflow-hidden">
            <TabBar />
            <AddressBar onNavigate={handleNavigate} />

            <div className="shell-body flex-1 flex overflow-hidden">
                <main className="content flex-1 relative bg-bg-primary">
                    {currentView === 'settings' ? (
                        <ModuleCenter />
                    ) : (
                        <div className="flex h-full items-center justify-center text-text-muted italic">
                            WebView Engine Active
                        </div>
                    )}
                </main>

                <aside className="w-12 border-l border-white/5 bg-bg-secondary/30 flex flex-col items-center py-4 gap-4">
                    {ModuleRegistry.getAllModules()
                        .filter(m => modules.find(ms => ms.metadata.id === m.metadata.id && ms.isActive))
                        .map(m => (
                            <button
                                key={m.metadata.id}
                                className="w-8 h-8 rounded-lg bg-accent-primary/10 flex items-center justify-center text-accent-primary hover:bg-accent-primary/20 transition-all border border-accent-primary/20"
                                title={m.metadata.name}
                            >
                                {m.metadata.icon || '🧩'}
                            </button>
                        ))
                    }
                </aside>
            </div>
        </div>
    );
}

