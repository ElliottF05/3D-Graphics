import * as CPPInterface from "./cppInterface";
import * as Database from "./database";
import * as LeftSideBar from "./leftSideBar";

// DOM elements
const imageGallery = document.getElementById('image-gallery') as HTMLDivElement;
let selectedImageContainer = document.getElementById('selected-image') as HTMLDivElement | null;
const viewImageButton = document.getElementById('view-image-button') as HTMLButtonElement;
const sceneStatusMessage = document.getElementById('scene-status-message') as HTMLDivElement;
const deleteImageButton = document.getElementById('delete-image-button') as HTMLButtonElement;
const saveImageButton = document.getElementById('save-image-button') as HTMLButtonElement;
const saveSceneButton = document.getElementById('save-scene-button') as HTMLButtonElement;
const sceneNameInput = document.getElementById('scene-name-input') as HTMLInputElement;
const imageButtonStatusMessage = document.getElementById('image-button-status-message') as HTMLDivElement;

let selectedImageUrl: string | null = null;
let selectedImageID: number | null = null;
let imageURLS: string[] = [];
let imageIDs: number[] = [];
let deletedImageIDs: number[] = [];


// EXPORT AND IMPORT SCENE DATA
function getCurrentSceneData(): Float32Array {
    console.log("Getting scene data...");
    // var data_buffer_size: number = CPPInterface.CPPgetDataBufferSize();
    // var data_buffer_pointer: number = CPPInterface.CPPgetDataBufferPointer();

    // var scene_data = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, data_buffer_pointer, data_buffer_size);
    // return scene_data;

    let dataBufferPointer: number = CPPInterface.CPPgetSceneDataBuffer();
    let dataBufferSize = (new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, dataBufferPointer, 1))[0];

    let dataBuffer = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, dataBufferPointer, dataBufferSize);

    return dataBuffer;
}

export async function uploadNewScene(sceneName: string, user_id: string): Promise<void>  {
    console.log("Adding new scene...");
    const { data, error } = await Database.supabase
        .from('scenes')
        .insert({ data: Array.from(getCurrentSceneData()), user_id: user_id, name: sceneName })
        .select()
    if (error) {
        console.error('Error inserting data:', error.message)
    } else {
        Database.setSceneID(data[0].id);
        await getImageFromCanvasAndUpload();
        LeftSideBar.renderScenes();
    }

}
export async function updateExistingScene(sceneName: string): Promise<void> {
    console.log("Updating scene...");
    const { error } = await Database.supabase
        .from('scenes')
        .update({ data: Array.from(getCurrentSceneData()), name: sceneName })
        .eq('id', Database.sceneID as number)
    if (error) {
        console.error('Error updating data:', error.message)
    } else {
        console.log("supabase updated with no error code");
        LeftSideBar.renderScenes();
    }
}

export async function uploadImage(image: Blob, scene_id: number, user_id: string): Promise<void> {
    console.log("Uploading image...");
    const url = "user:" + user_id + "date:" + Date.now();
    const { data, error } = await Database.supabase.storage
        .from('images')
        .upload(url, image)
    if (error) {
        console.error('Error uploading image:', error.message)
    } else {
        console.log('Image uploaded:', data);
        const { error } = await Database.supabase
            .from('images')
            .insert({ url: url, scene_id: scene_id })
        if (error) {
            console.log("Failed to insert new image into image table");
        }
    }
}

export async function deleteImage(image_id: number): Promise<void> {
    const { data, error } = await Database.supabase
        .from('images')
        .select('url')
        .eq('id', image_id)
    if (error) {
        console.error('Error fetching image url for deletion:', error.message)
    } else {
        console.log("deleting image with url:", data[0].url);
        deletedImageIDs.push(image_id);
        const url = data[0].url;
        await Database.supabase
            .from('images')
            .delete()
            .eq('id', image_id)
        const { error } = await Database.supabase.storage
            .from('images')
            .remove([url])
        if (error) {
            console.error('Error deleting image from storage:', error.message)
        }
    }
}


// Function to render images into the gallery
export async function renderImageGallery() {
    console.log("Rendering image gallery");

    imageURLS = [];
    imageIDs = [];
    if (Database.sceneID !== null) {
        const { data, error } = await Database.supabase
            .from('images')
            .select('url, id')
            .eq('scene_id', Database.sceneID)
            .order('created_at', { ascending: false })
        if (error) {
            console.error('Error fetching images:', error.message)
        } else {
            imageURLS = data.map((image: any) => Database.publicImgUrl + image.url);
            imageIDs = data.map((image: any) => image.id);
        }
    }

    imageGallery.innerHTML = ''; // Clear the gallery
    for (let i = 0; i < imageURLS.length; i++) {
        const imageUrl = imageURLS[i];
        const imageID = imageIDs[i];

        if (imageID in deletedImageIDs) {
            continue;
        }

        const imageItem = document.createElement('div');
        imageItem.classList.add('image-item');

        const img = document.createElement('img');
        img.crossOrigin = 'anonymous';
        img.src = imageUrl;
        img.alt = 'Rendered Image';
        img.addEventListener('click', () => handleImageClick(imageUrl, imageID, imageItem));

        imageItem.appendChild(img);
        imageGallery.appendChild(imageItem);
    }
}
export function setSceneInputName(sceneName: string): void {
    sceneNameInput.value = sceneName;
}
function setSceneStatusMessage(message: string): void {
    sceneStatusMessage.innerText = message;
    setTimeout(() => {sceneStatusMessage.innerText = ''}, 5000);
}

// Handle clicking an image
function handleImageClick(imageUrl: string, imageID: number, imageItem: HTMLDivElement) {
    console.log("image clicked");
    if (selectedImageContainer) {
        selectedImageContainer.classList.remove('selected-image'); // Reset the previously selected image section
    }
    selectedImageUrl = imageUrl;
    selectedImageID = imageID;
    selectedImageContainer = imageItem;
    selectedImageContainer.classList.add('selected-image'); // Show the selected image section
}

// Handle "View Image" button click
viewImageButton.addEventListener('click', () => {
    if (selectedImageUrl) {
        window.open(selectedImageUrl, '_blank'); // Open in a new tab
    }
});

// Handle "Delete Image" button click
deleteImageButton.addEventListener('click', async () => {
    console.log("delete image clicked");
    if (selectedImageID) {
        if (imageURLS.length === 1) {
            imageButtonStatusMessage.innerText = 'Your scene must have at least one image';
            setTimeout(() => {imageButtonStatusMessage.innerText = ''}, 5000);
            return;
        }
        await deleteImage(selectedImageID);
        await renderImageGallery();
        selectedImageUrl = null; // Reset selected image
        selectedImageContainer = null;
    }
});

async function getImageFromCanvasAndUpload() {
    const canvas = document.getElementById('canvas') as HTMLCanvasElement;
    let imageBlob: Blob;
    let uploaded = false;
    canvas.toBlob(async (blob) => {
        if (blob) {
            imageBlob = blob;
            await uploadImage(imageBlob, Database.sceneID as number, Database.user?.id as string);
            uploaded = true;
            renderImageGallery();
        }
    });
    while (!uploaded) {
        await new Promise(r => setTimeout(r, 10));
    }
}
saveImageButton.addEventListener('click', async () => {
    if (!Database.getUserSignedIn()) {
        setSceneStatusMessage('You must be signed in to save images');
        return;
    }
    if (Database.sceneID === null) {
        setSceneStatusMessage('You must save the scene to save images');
        return;
    }
    getImageFromCanvasAndUpload();
});

saveSceneButton.addEventListener('click', async () => {
    if (!Database.getUserSignedIn()) {
        setSceneStatusMessage('You must be signed in to save scenes');
        return;
    }
    if (sceneNameInput.value === '') {
        setSceneStatusMessage('You must enter a scene name to save the scene');
        return;
    }

    const sceneName = sceneNameInput.value;

    if (Database.sceneID === null) { // create scene
        uploadNewScene(sceneName, Database.user?.id as string);
    } else { // update existing scene
        updateExistingScene(sceneName);
    };
});