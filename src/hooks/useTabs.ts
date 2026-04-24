import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export interface Tab {
    id: string;
    url: string;
    title: string;
    active: boolean;
}

export function useTabs() {
    const [tabs, setTabs] = useState<Tab[]>([
        { id: 'initial', url: 'https://google.com', title: 'Google', active: true }
    ]);

    const activeTab = tabs.find(t => t.active);

    const addTab = useCallback(async (url: string = 'https://google.com') => {
        try {
            const id = await invoke<string>('create_tab', { url });
            setTabs(prev => {
                const deactivated = prev.map(t => ({ ...t, active: false }));
                return [...deactivated, { id, url, title: 'New Tab', active: true }];
            });
            return id;
        } catch (error) {
            console.error('Failed to create tab:', error);
            return null;
        }
    }, []);

    const removeTab = useCallback(async (id: string) => {
        try {
            await invoke('close_tab', { id });
            setTabs(prev => {
                const filtered = prev.filter(t => t.id !== id);
                if (filtered.length === 0) {
                    return [{ id: 'fallback', url: 'https://google.com', title: 'New Tab', active: true }];
                }
                if (prev.find(t => t.id === id)?.active) {
                    filtered[filtered.length - 1].active = true;
                }
                return filtered;
            });
        } catch (error) {
            console.error('Failed to close tab:', error);
        }
    }, []);

    const switchTab = useCallback((id: string) => {
        setTabs(prev => prev.map(t => ({
            ...t,
            active: t.id === id
        })));
    }, []);

    const updateTabUrl = useCallback((id: string, url: string) => {
        setTabs(prev => prev.map(t => t.id === id ? { ...t, url, title: url } : t));
    }, []);

    return {
        tabs,
        activeTab,
        addTab,
        removeTab,
        switchTab,
        updateTabUrl
    };
}
