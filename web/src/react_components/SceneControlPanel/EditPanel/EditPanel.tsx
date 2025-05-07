import React from 'react';
import { Accordion } from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { Trash2 } from 'lucide-react';
import TransformControls from './TransformControls';
import MaterialEditorControls from './MaterialEditorControls';

const wasmDeleteSelectedObject = () => {
    console.log("WASM: Delete selected object");
};

interface EditPanelProps {
    initialOpenSections?: string[]; // Sections to open by default
    accordionKey?: string; // Key to force re-render of accordion if needed
}

const EditPanel: React.FC<EditPanelProps> = ({ 
    initialOpenSections = ['transform', 'materialEditor'], // Default to only transform open
    accordionKey 
}) => {
    const handleDeleteObject = () => {
        if (window.confirm("Are you sure you want to delete this object?")) {
            wasmDeleteSelectedObject();
        }
    };

    return (
        <div className="space-y-3">
            <Button 
                variant="destructive" 
                className="w-full" 
                onClick={handleDeleteObject}
            >
                <Trash2 className="mr-2 h-4 w-4" />
                Delete Selected Object
            </Button>
            
            <Accordion 
                type="multiple" 
                defaultValue={initialOpenSections} 
                className="w-full"
                key={accordionKey} // Use the key here
            >
                <TransformControls />
                <MaterialEditorControls />
            </Accordion>
        </div>
    );
};

export default EditPanel;