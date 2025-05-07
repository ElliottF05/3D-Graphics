import React from 'react';
import { Accordion } from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { Trash2, CheckCircle2 } from 'lucide-react';
import TransformControls from './TransformControls';
import MaterialEditorControls from './MaterialEditorControls';

const wasmDeleteSelectedObject = () => {
    console.log("WASM: Delete selected object");
};

const wasmConfirmObjectEdits = () => {
    console.log("WASM: Confirming edits for selected object");
};

interface EditPanelProps {
    initialOpenSections?: string[];
    accordionKey?: string;
    onConfirmEdit: () => void;
    onDeleteObject: () => void;
}

const EditPanel: React.FC<EditPanelProps> = ({ 
    initialOpenSections = ['transform', 'materialEditor'],
    accordionKey,
    onConfirmEdit,
    onDeleteObject
}) => {
    const handleDeleteClick = () => {
        if (window.confirm("Are you sure you want to delete this object?")) {
            wasmDeleteSelectedObject();
            onDeleteObject(); // Notify parent
        }
    };

    const handleConfirmClick = () => {
        wasmConfirmObjectEdits();
        onConfirmEdit(); // Notify parent to close panel and deselect
    };

    return (
        <div className="space-y-3">
            <div className="flex space-x-2"> {/* Flex container for buttons */}
                <Button 
                    variant="default" // Default variant is often blue/primary, can be customized
                    className="w-1/2 bg-green-600 hover:bg-green-700 text-white" // Green button, half width
                    onClick={handleConfirmClick}
                >
                    <CheckCircle2 className="mr-2 h-4 w-4" />
                    Confirm Edit
                </Button>
                <Button 
                    variant="destructive" 
                    className="w-1/2" // Half width
                    onClick={handleDeleteClick}
                >
                    <Trash2 className="mr-2 h-4 w-4" />
                    Delete Object
                </Button>
            </div>
            
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