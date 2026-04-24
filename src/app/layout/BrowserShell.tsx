/**
 * Copyright (c) 2026 Simon Project
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

import React from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { TabBar } from './TabBar';
import { AddressBar } from './AddressBar';
import { useModules } from '../../hooks/useModules';
import { useTabs } from '../../hooks/useTabs';
import { ModuleRegistry } from '../modules/registry';
import { ModuleCenter } from '../../components/Settings/ModuleCenter';

export function BrowserShell() {
    const { modules } = useModules();
    const { tabs, activeTab, addTab, removeTab, switchTab, updateTabUrl } = useTabs();
    const [currentView, setCurrentView] = React.useState<'browser' | 'settings'>('browser');

    const handleNavigate = async (url: string) => {
        if (url === 'bluecore://settings') {
            setCurrentView('settings');
            return;
        }
        setCurrentView('browser');
        if (activeTab) {
            try {
                await invoke('bc_handle_navigation', { tabId: activeTab.id, url });
                updateTabUrl(activeTab.id, url);
            } catch (error) {
                console.error("Navigation Bridge Error:", error);
            }
        }
    };

    return (
        <div id="bluecore-shell" className="flex flex-col h-screen bg-bg-primary overflow-hidden">
            <TabBar
                tabs={tabs}
                onAddTab={() => addTab()}
                onRemoveTab={removeTab}
                onSwitchTab={switchTab}
            />
            <AddressBar onNavigate={handleNavigate} />

            <div className="shell-body flex-1 flex overflow-hidden">
                <main className="content flex-1 relative bg-bg-primary">
                    {currentView === 'settings' ? (
                        <ModuleCenter />
                    ) : (
                        <div className="flex h-full items-center justify-center text-text-muted italic flex-col gap-4">
                            <div className="text-4xl opacity-20 font-bold">BlueCore</div>
                            <div className="flex flex-col items-center gap-1">
                                <span className="text-sm border border-white/5 px-3 py-1 rounded-full bg-white/5">{activeTab?.url || 'No Active Tab'}</span>
                                <span className="text-xs opacity-40">WebView Engine Active</span>
                            </div>
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

