import * as CPPInterface from './cppInterface.js';
import { createClient } from '@supabase/supabase-js'
import { Database, Json } from './database.types'

const SUPABASE_URL: string = import.meta.env.VITE_SUPABASE_URL;
const SUPABASE_API_KEY: string = import.meta.env.VITE_SUPABASE_API_KEY;

const supabase = createClient<Database>(
    SUPABASE_URL,
    SUPABASE_API_KEY
)

// Exporting scene data
export async function test(): Promise<void> {
    console.log('Test function called');
    console.log(buildSceneData());
    
    let test_json: Json = <Json><unknown> {
        "test_key": "test_value"
    };

    const { error } = await supabase
        .from('test')
        .insert({ id: 1, column1: test_json })

}

function buildSceneData(): JSON {
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

    var scene_data = <JSON><unknown> {
        'metadata' : Array.from(metadata),
        'pos_data' : Array.from(pos_data),
        'color_data' : Array.from(color_data)
    }
    // console.log(scene_data);

    return scene_data;
}