import React from 'react';
import { Accordion } from "@/components/ui/accordion";
import { Button } from "@/components/ui/button"; // Import Button
import { Trash2 } from 'lucide-react'; // Optional: for an icon
import TransformControls from './TransformControls';
import MaterialEditorControls from './MaterialEditorControls';

// Mock WASM function call for deleting an object
const wasmDeleteSelectedObject = () => {
    console.log("WASM: Delete selected object");
    // Example: Module.ccall('delete_selected_object', null, [], []);
};

const EditPanel: React.FC = () => {
    const handleDeleteObject = () => {
        // Add confirmation dialog here if needed
        if (window.confirm("Are you sure you want to delete this object?")) {
            // Call the WASM function to delete the selected object
            wasmDeleteSelectedObject();
        }
    };

    return (
        <div className="space-y-3"> {/* Added space-y-3 for spacing between button and accordion */}
            <Button 
                variant="destructive" 
                className="w-full" 
                onClick={handleDeleteObject}
            >
                <Trash2 className="mr-2 h-4 w-4" /> {/* Optional Icon */}
                Delete Selected Object
            </Button>
            
            <Accordion type="multiple" defaultValue={['transform']} className="w-full">
                <TransformControls />
                <MaterialEditorControls />
            </Accordion>
        </div>
    );
};

export default EditPanel;