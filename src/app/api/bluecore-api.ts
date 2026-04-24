/**
 * Copyright (c) 2026 Simon Project
 * Licensed under the Apache License, Version 2.0
 * Part of the Simon Project BlueCore Browser
 */

import { invoke } from '@tauri-apps/api/tauri';

export const BlueCoreAPI = {
    tabs: {
        create: (url: string): Promise<string> => invoke('create_tab', { url }),
        close: (id: string): Promise<boolean> => invoke('close_tab', { id }),
    },
    engine: {
        switch: (name: string): Promise<string> => invoke('switch_engine', { name }),
    },
    identity: {
        getSession: (): Promise<{ session_id: string; username: string | null }> => invoke('get_session'),
        login: (username: string): Promise<void> => invoke('login', { username }),
        logout: (): Promise<void> => invoke('logout'),
    }
};
