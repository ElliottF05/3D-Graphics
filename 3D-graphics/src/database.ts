import { Database } from './database.types'
import { createClient, User } from '@supabase/supabase-js'
import './leftSideBar.ts'
import './rightSideBar.ts'

// INITIALIZING SUPABASE
const SUPABASE_URL: string = import.meta.env.VITE_SUPABASE_URL;
const SUPABASE_API_KEY: string = import.meta.env.VITE_SUPABASE_API_KEY;

export const supabase = createClient<Database>(
    SUPABASE_URL,
    SUPABASE_API_KEY
)
export const publicImgUrl = "https://ehlqrserkbegsvjfvoze.supabase.co/storage/v1/object/public/images/";
export let user: null | User = null;
export let sceneID: number | null = null;

export function getUserSignedIn(): boolean {
    return user !== null;
}
export function setUser(new_user: User | null): void {
    user = new_user;
}
export function setSceneID(new_sceneID: number | null): void {
    sceneID = new_sceneID;
}