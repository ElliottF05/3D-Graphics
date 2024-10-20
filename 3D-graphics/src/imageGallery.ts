import * as Database from './database';
import { AuthError } from '@supabase/supabase-js';

const sceneNameInput = document.getElementById('scene-name-input') as HTMLInputElement;
const saveImageButton = document.getElementById('save-image-button') as HTMLButtonElement;
const saveSceneButton = document.getElementById('save-scene-button') as HTMLButtonElement;

let sceneID: number | null = null;

saveImageButton.addEventListener('click', async () => {
    if (!Database.getUserSignedIn()) {
        sceneStatusMessage.innerText = 'You must be signed in to save images';
        return;
    }
    if (sceneID === null) {
        sceneStatusMessage.innerText = 'You must save the scene to save images';
        return;
    }
    const canvas = document.getElementById('canvas') as HTMLCanvasElement;
    let imageSrc;
    let imageBlob: Blob;
    canvas.toBlob((blob) => {
        if (blob) {
            imageBlob = blob;
            imageSrc = URL.createObjectURL(blob);
            imageURLS.unshift(imageSrc);
            Database.uploadImage(imageBlob, sceneID as number);
            renderImageGallery();
        }
    });
    sceneStatusMessage.innerText = '';
});

saveSceneButton.addEventListener('click', async () => {
    if (!Database.getUserSignedIn()) {
        sceneStatusMessage.innerText = 'You must be signed in to save scenes';
        return;
    }
    if (sceneNameInput.value === '') {
        sceneStatusMessage.innerText = 'You must enter a scene name to save the scene';
        return;
    }
    const sceneName = sceneNameInput.value;
    console.log("SCENENAME:", sceneName);
    sceneID = await Database.addOrUpdateScene(sceneID, sceneName);
    if (sceneID === -1) {
        sceneStatusMessage.innerText = 'Error saving scene';
    } else {
        saveImageButton.click();
        sceneStatusMessage.innerText = '';
    }
});



const imageGallery = document.getElementById('image-gallery') as HTMLDivElement;
let selectedImageContainer = document.getElementById('selected-image') as HTMLDivElement | null;
const viewImageButton = document.getElementById('view-image-button') as HTMLButtonElement;
const sceneStatusMessage = document.getElementById('scene-status-message') as HTMLDivElement;
// const deleteImageButton = document.getElementById('deleteImageButton') as HTMLButtonElement;

let selectedImageUrl: string | null = null;
let imageURLS: string[] = [];
imageURLS.push('scene.png');
imageURLS.push('scene.png');
imageURLS.push('scene.png');

// Initial render of images
renderImageGallery();

// Function to render images into the gallery
async function renderImageGallery() {
    console.log("rendering images");
    // const images = await fetchImages(); // Fetch images from Supabase
    imageGallery.innerHTML = ''; // Clear the gallery
    imageURLS.forEach((imageUrl: string) => {
        const imageItem = document.createElement('div');
        imageItem.classList.add('image-item');

        const img = document.createElement('img');
        img.src = imageUrl;
        img.alt = 'Rendered Image';
        img.addEventListener('click', () => handleImageClick(imageUrl, imageItem));

        imageItem.appendChild(img);
        imageGallery.appendChild(imageItem);
    });
}

// Handle clicking an image
function handleImageClick(imageUrl: string, imageItem: HTMLDivElement) {
    console.log("image clicked");
    if (selectedImageContainer) {
        selectedImageContainer.classList.remove('selected-image'); // Reset the previously selected image section
    }
    selectedImageUrl = imageUrl;
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
// deleteImageButton.addEventListener('click', async () => {
//     if (selectedImageUrl) {
//         // await deleteImage(selectedImageUrl); // Delete the image from the database
//         imageGallery.innerHTML = ''; // Clear the gallery and re-render without the deleted image
//         await renderImages();
//         // selectedImageContainer.style.display = 'none'; // Hide the selected image section
//         selectedImageUrl = null; // Reset selected image
//     }
// });