import * as CPPInterface from './cppInterface.js';
import { AuthError, createClient, User } from '@supabase/supabase-js'
import { Database } from './database.types'

// DOM ELEMENTS
const signInButton = document.getElementById('sign-in-button') as HTMLButtonElement;
const signUpButton = document.getElementById('sign-up-button') as HTMLButtonElement;
const emailField = document.getElementById('email') as HTMLInputElement;
const passwordField = document.getElementById('password') as HTMLInputElement;
const statusMessage = document.getElementById('user-status-message') as HTMLDivElement;


// INITIALIZING SUPABASE
const SUPABASE_URL: string = import.meta.env.VITE_SUPABASE_URL;
const SUPABASE_API_KEY: string = import.meta.env.VITE_SUPABASE_API_KEY;

const supabase = createClient<Database>(
    SUPABASE_URL,
    SUPABASE_API_KEY
)
let user: null | User = null;
let userID: number | null = null;
let userLoggedIn: boolean = false;


// TEST
export async function test(): Promise<void> {
    console.log('Test function called');

    console.log('Testing exportSceneData()');
    addOrUpdateScene(null, "testName");

}


// AUTH
export function getUserID(): number {
    return Number(user?.id);
}
export function getUserSignedIn(): boolean {
    return userLoggedIn;
}

export async function signUp(email: string, password: string): Promise<void> {
    console.log('Signing up...');
    const { data, error } = await supabase.auth.signUp({
        email: email,
        password: password
    })
    if (error) {
        handleAuthError(error, 'signUp');
        console.error('Error signing up:', error.message)
    } else {
        console.log('Signed up:', data);
        user = data.user;
        userLoggedIn = true;
        onAuthSuccess();
    }
}
export async function signIn(email: string, password: string): Promise<void> {
    console.log('Signing in...');
    const { data, error } = await supabase.auth.signInWithPassword({
        email: email,
        password: password
    })
    if (error) {
        handleAuthError(error, 'signIn');
        console.error('Error signing up:', error.message)
    } else {
        console.log('Signed in:', data);
        user = data.user;
        userLoggedIn = true;
        onAuthSuccess();
    }
}

function handleAuthError(error: AuthError, from: string): void {
    console.log("Auth error code:", error.code);
    statusMessage.className = 'error';
    if (error.code == "invalid_credentials") {
        if (from == 'signUp') {
            statusMessage.innerText = 'Invalid email or password';
        } else {
            statusMessage.innerText = 'Incorrect or invalid email or password';
        }
    }
    else if (error.code == "validation_failed") {
        statusMessage.innerText = 'Invalid email or password';
    } 
    else if (error.code == "anonymous_provider_disabled") {
        statusMessage.innerText = 'Invalid email or password';
    }
    else if (error.code == "user_already_exists") {
        statusMessage.innerText = 'Email already in use';
    }
    else {
        statusMessage.innerText = 'An error occurred';
    }
}

function onAuthSuccess() {
    console.log('User logged in:', user);
    statusMessage.innerText = 'Welcome, ' + user?.email;
    statusMessage.className = '';
}

// Event listeners for sign-in and sign-up buttons
signInButton.addEventListener('click', async () => {
    const email = emailField.value;
    const password = passwordField.value;
    await signIn(email, password);
});

signUpButton.addEventListener('click', async () => {
    const email = emailField.value;
    const password = passwordField.value;
    await signUp(email, password);
});


// EXPORT AND IMPORT SCENE DATA
function getSceneData(): Float32Array {
    console.log("Getting scene data...");
    var data_buffer_size: number = CPPInterface.CPPgetDataBufferSize();
    var data_buffer_pointer: number = CPPInterface.CPPgetDataBufferPointer();

    var scene_data = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, data_buffer_pointer, data_buffer_size);
    return scene_data;

}

export async function addOrUpdateScene(sceneID: number | null, sceneName: string): Promise<number> {
    let scene_data = getSceneData();

    if (sceneID === null) {
        console.log("Adding new scene...");
        console.log("USERID: ", user?.id);
        console.log("SCENENAME: ", sceneName);
        const { data, error } = await supabase
            .from('scenes')
            .insert({ data: Array.from(scene_data), user_id: user?.id as string, name: sceneName })
            .select()
        if (error) {
            console.error('Error inserting data:', error.message)
        } else {
            return data[0].id;
        }

    } else {
        console.log("Updating scene...");
        const { data, error } = await supabase
            .from('scenes')
            .update({ data: Array.from(scene_data), name: sceneName })
            .eq('id', sceneID)
        if (error) {
            console.error('Error updating data:', error.message)
        } else {
            return sceneID;
        }
    }
    return -1;
}
export async function importSceneData(sceneID: number): Promise<void> {
    console.log("Importing scene data...");
    const { data, error } = await supabase
        .from('scenes')
        .select('data')
        .eq('id', sceneID) 
    if (error) {
        console.error('Error fetching data:', error.message)
        return;
    } else {
        // console.log(data)
        let scene_data = data[0].data;
        console.log(scene_data);

        let size = scene_data.length;

        let data_buffer_pointer = CPPInterface.CPPsetDataBufferPointer(size);
        let data_buffer = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, data_buffer_pointer, scene_data.length);
        for (let i = 0; i < data_buffer.length; i++) {
            data_buffer[i] = scene_data[i];
        }

        CPPInterface.CPPloadScene(data_buffer_pointer);

    }
}

export async function uploadImage(image: Blob, scene_id: number): Promise<void> {
    console.log("Uploading image...");
    const url = String(user?.id) + Date.now();
    const { data, error } = await supabase.storage.from('scene_images').upload(url, image)
    if (error) {
        console.error('Error uploading image:', error.message)
    } else {
        console.log('Image uploaded:', data);
        const { error } = await supabase
            .from('images')
            .insert({ url: url, scene_id: scene_id })
        if (error) {
            console.log("Failed to insert new image into image table");
        }
    }
}
