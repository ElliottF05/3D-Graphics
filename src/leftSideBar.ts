import * as Database from './database';
import * as CPPInterface from './cppInterface';
import * as RightSideBar from './rightSideBar';
import { AuthError } from '@supabase/supabase-js';

// DOM ELEMENTS
const statusMessage = document.getElementById('user-status-message') as HTMLDivElement;
const signInButton = document.getElementById('sign-in-button') as HTMLButtonElement;
const signUpButton = document.getElementById('sign-up-button') as HTMLButtonElement;
const emailField = document.getElementById('email') as HTMLInputElement;
const passwordField = document.getElementById('password') as HTMLInputElement;

const sceneGallery = document.getElementById('scene-gallery') as HTMLDivElement;
const sceneGalleryButtons = document.getElementById('scene-gallery-buttons') as HTMLDivElement;

const deleteSceneButton = document.getElementById('delete-scene-button') as HTMLButtonElement;
const loadSceneButton = document.getElementById('load-scene-button') as HTMLButtonElement;

const scenes = [
    { name: 'Scene 1', thumbnailUrl: 'path_to_thumbnail1.png', sceneID: 1 },
    { name: 'Scene 2', thumbnailUrl: 'path_to_thumbnail2.png', sceneID: 2 },
];
let selectedSceneID: number | null = null;
let selectedSceneContainer: HTMLDivElement | null = null;



export async function signUp(email: string, password: string): Promise<void> {
    console.log('Signing up...');
    const { data, error } = await Database.supabase.auth.signUp({
        email: email,
        password: password
    })
    if (error) {
        handleAuthError(error, 'signUp');
        console.error('Error signing up:', error.message)
    } else {
        console.log('Signed up:', data);
        Database.setUser(data.user);
        onAuthSuccess(data.user?.email as string);
    }
}
export async function signIn(email: string, password: string): Promise<void> {
    console.log('Signing in...');
    const { data, error } = await Database.supabase.auth.signInWithPassword({
        email: email,
        password: password
    })
    if (error) {
        handleAuthError(error, 'signIn');
        console.error('Error signing up:', error.message)
    } else {
        console.log('Signed in:', data);
        Database.setUser(data.user);
        onAuthSuccess(data.user.email as string);
    }
    renderScenes();
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

function onAuthSuccess(user_email: string): void {
    console.log('User logged in:', user_email);
    statusMessage.innerText = 'Welcome, ' + user_email;
    statusMessage.className = '';
    sceneGalleryButtons.style.display = 'block';
}

export async function importSceneData(sceneID: number): Promise<void> {
    console.log("Importing scene data...");
    const { data, error } = await Database.supabase
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



// Function to render scenes into the gallery
async function getThumbnails(scene_ids: number[]) {
    const { data, error } = await Database.supabase
        .from('images')
        .select('scene_id, url, created_at')
        .in('scene_id', scene_ids)
        .order('created_at', { ascending: false })
    if (error) {
        console.error('Error fetching from image table:', error.message)
    } else {
        const seen: number[] = [];
        const new_data = data.filter((image) => {
            if (seen.includes(image.scene_id)) {
                return false;
            } else {
                seen.push(image.scene_id);
                return true;
            }
        });
        return new_data;
    }
    return null;
}
export async function renderScenes() {

    sceneGallery.innerHTML = ''; // Clear the gallery
    scenes.length = 0;

    if (Database.user === null) {
        console.log("User not logged in");
        return;
    }

    const { data, error } = await Database.supabase
        .from('scenes')
        .select('id, name')
        .eq('user_id', Database.user?.id as string)
        .order('last_edited', { ascending: false })
    if (error) {
        console.error('Error fetching from scene table:', error.message)
        return;
    } else {
        const scene_ids = data.map((scene) => scene.id);
        const thumbnail_data = await getThumbnails(scene_ids);

        if (thumbnail_data === null) {
            console.log("Error fetching thumbnail data");
            return;
        }

        scenes.length = 0;
        data.forEach((scene) => {
            const thumbnail = thumbnail_data.find((image) => image.scene_id == scene.id);
            scenes.push({
                name: scene.name as string,
                thumbnailUrl: thumbnail?.url as string,
                sceneID: scene.id
            });
        });
    }

    scenes.forEach((scene) => {
        // Create scene item container
        const sceneItem = document.createElement('div');
        sceneItem.classList.add('scene-item');
        
        // Create and append the scene name
        const sceneName = document.createElement('p');
        sceneName.textContent = scene.name;
        sceneItem.appendChild(sceneName);
        
        // Create and append the scene thumbnail
        const thumbnail = document.createElement('img');
        thumbnail.src = Database.publicImgUrl + scene.thumbnailUrl;
        thumbnail.alt = `Thumbnail of ${scene.name}`;
        thumbnail.classList.add('scene-thumbnail');
        thumbnail.crossOrigin = 'anonymous';
        sceneItem.appendChild(thumbnail);

        // Add click functionality
        sceneItem.addEventListener('click', () => handleSceneClick(scene.sceneID, sceneItem));

        // Append the scene item to the gallery
        sceneGallery.appendChild(sceneItem);
    });
}

function handleSceneClick(sceneID: number, sceneContainer: HTMLDivElement): void {
    if (selectedSceneContainer !== null) {
        selectedSceneContainer.classList.remove('scene-selected');
    }
    sceneContainer.classList.add('scene-selected');
    selectedSceneID = sceneID;
    selectedSceneContainer = sceneContainer;
}

deleteSceneButton.addEventListener('click', async () => {
    if (selectedSceneID === null) {
        console.log('No scene selected');
        return;
    }
    console.log('Deleting scene:', selectedSceneID);

    const { error } = await Database.supabase
        .from('scenes')
        .delete()
        .eq('id', selectedSceneID)
    if (error) {
        console.error('Error deleting scene:', error.message)
    } else {
        console.log('Deleted scene: ', selectedSceneID);
        const { data, error } = await Database.supabase
            .from('images')
            .delete()
            .eq('scene_id', selectedSceneID as number)
            .select();
        if (error) {
            console.log("Error deleting images from image table:", error.message);
        } else {
            if (data.length > 0) {
                const { error } = await Database.supabase.storage
                    .from('images')
                    .remove(data.map((image: any) => image.url))
                if (error) {
                    console.error('Error deleting images from storage:', error.message)
                }
            }
        }


        if (selectedSceneID === Database.sceneID) {
            Database.setSceneID(null);
            RightSideBar.renderImageGallery();
        }
        selectedSceneID = null;
        selectedSceneContainer = null;
        renderScenes();
    }
});

loadSceneButton.addEventListener('click', async () => {
    if (selectedSceneID === null) {
        console.log("Can't load scene, no scene selected");
        return;
    }
    console.log('Loading scene:', selectedSceneID);
    await importSceneData(selectedSceneID);
    Database.setSceneID(selectedSceneID);
    RightSideBar.renderImageGallery();
    const { data, error } = await Database.supabase
        .from('scenes')
        .select('name')
        .eq('id', selectedSceneID)
    if (error) {
        console.error('Error fetching scene name:', error.message)
    } else {
        RightSideBar.setSceneInputName(data[0].name as string);
    }
});