/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 */

import React from 'react';
import { Tab } from '../../hooks/useTabs';

interface TabBarProps {
    tabs: Tab[];
    onAddTab: () => void;
    onRemoveTab: (id: string) => void;
    onSwitchTab: (id: string) => void;
}

export function TabBar({ tabs, onAddTab, onRemoveTab, onSwitchTab }: TabBarProps) {
    return (
        <div className="tab-bar-container glass flex items-center px-4 gap-2 h-10 border-b border-white/5 bg-bg-secondary/50 backdrop-blur-md">
            <div className="tabs flex-1 flex gap-1 overflow-x-auto no-scrollbar">
                {tabs.map((tab) => (
                    <div
                        key={tab.id}
                        onClick={() => onSwitchTab(tab.id)}
                        className={`tab-item group flex items-center gap-2 px-3 py-1 rounded-t-lg transition-all cursor-pointer min-w-[120px] max-w-[200px] border-t border-x border-transparent ${tab.active
                                ? 'bg-bg-primary border-white/10 text-accent-primary'
                                : 'hover:bg-white/5 text-text-muted hover:text-text-primary'
                            }`}
                    >
                        <span className="tab-title text-xs truncate flex-1">{tab.title}</span>
                        <button
                            className="tab-close opacity-0 group-hover:opacity-100 hover:bg-white/10 rounded-full w-4 h-4 flex items-center justify-center text-xs transition-all"
                            onClick={(e) => {
                                e.stopPropagation();
                                onRemoveTab(tab.id);
                            }}
                        >
                            ×
                        </button>
                    </div>
                ))}
            </div>
            <button
                onClick={onAddTab}
                className="new-tab-btn w-6 h-6 flex items-center justify-center rounded-lg hover:bg-white/5 text-text-muted hover:text-accent-primary transition-all ml-2"
                title="New Tab"
            >
                +
            </button>
        </div>
    );
}
