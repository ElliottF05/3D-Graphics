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
    exportSceneData();

}

async function exportSceneData(): Promise<void> {
    console.log("Getting scene data...");

    var metadata_size: number = CPPInterface.CPPgetSceneMetaDataSize();
    var metadata_buffer_pointer: number = CPPInterface.CPPgetSceneMetaDataBuffer();

    var metadata = new Uint32Array(CPPInterface.CPPmodule.HEAPU32.buffer, metadata_buffer_pointer, metadata_size);

    var num_objects: number = metadata[0];
    var object_sizes: number[] = [];
    var num_triangles: number = 0;
    for (let i = 0; i < num_objects; i++) {
        object_sizes.push(metadata[1 + i]);
        num_triangles += metadata[1 + i];
    }

    var pos_buffer_pointer: number = CPPInterface.CPPgetScenePosDataBuffer();
    var color_buffer_pointer: number = CPPInterface.CPPgetSceneColorDataBuffer();

    var pos_data = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, pos_buffer_pointer, num_triangles * 9);
    var color_data = new Uint32Array(CPPInterface.CPPmodule.HEAPU32.buffer, color_buffer_pointer, num_triangles * 3);

    const { error } = await supabase
        .from('test')
        .insert({ metadata: Array.from(metadata), pos_data: Array.from(pos_data), color_data: Array.from(color_data) })
    if (error) {
        console.error('Error inserting data:', error.message)
    }

    
}

export async function importSceneData(sceneID: number): Promise<void> {
    const { data, error } = await supabase
        .from('test')
        .select('metadata, pos_data, color_data')
        .eq('id', sceneID) 
    if (error) {
        console.error('Error fetching data:', error.message)
        return;
    } else {
        console.log(data)
        let metadata = data[0].metadata;
        let pos_data = data[0].pos_data;
        let color_data = data[0].color_data;
        console.log(metadata);
        console.log(pos_data);
        console.log(color_data);

        let size = 0;
        size += metadata.length;
        size += pos_data.length;
        size += color_data.length;

        let metadata_buffer_pointer = CPPInterface.CPPsetSceneDataBuffer(size);
        let pos_buffer_pointer = metadata_buffer_pointer + 4 * metadata.length;
        let color_buffer_pointer = pos_buffer_pointer + 4 * pos_data.length;

        let metadata_buffer = new Uint32Array(CPPInterface.CPPmodule.HEAPU32.buffer, metadata_buffer_pointer, metadata.length);
        for (let i = 0; i < metadata_buffer.length; i++) {
            metadata_buffer[i] = metadata[i];
        }

        let pos_data_buffer = new Float32Array(CPPInterface.CPPmodule.HEAPF32.buffer, pos_buffer_pointer, pos_data.length);
        for (let i = 0; i < pos_data_buffer.length; i++) {
            pos_data_buffer[i] = pos_data[i];
        }

        let color_data_buffer = new Uint32Array(CPPInterface.CPPmodule.HEAPU32.buffer, color_buffer_pointer, color_data.length);
        for (let i = 0; i < color_data_buffer.length; i++) {
            color_data_buffer[i] = color_data[i];
        }

        CPPInterface.CPPloadScene(metadata_buffer_pointer, pos_buffer_pointer, color_buffer_pointer);

    }
}
