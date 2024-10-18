import * as CPPInterface from './cppInterface.js';
import { createClient } from '@supabase/supabase-js'
import { Database } from './database.types'

const SUPABASE_URL: string = import.meta.env.VITE_SUPABASE_URL;
const SUPABASE_API_KEY: string = import.meta.env.VITE_SUPABASE_API_KEY;

const supabase = createClient<Database>(
    SUPABASE_URL,
    SUPABASE_API_KEY
)

// Exporting scene data
export async function test(): Promise<void> {
    console.log('Test function called');

    console.log('Testing exportSceneData()');
    exportSceneData();

}

async function exportSceneData(): Promise<void> {
    console.log("Exporting scene data...");

    var data_buffer_size: number = CPPInterface.CPPgetDataBufferSize();
    var data_buffer_pointer: number = CPPInterface.CPPgetDataBufferPointer();

    var scene_data = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, data_buffer_pointer, data_buffer_size);

    const { error } = await supabase
        .from('scenes')
        .insert({ data: Array.from(scene_data) })
    if (error) {
        console.error('Error inserting data:', error.message)
    }

    
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
