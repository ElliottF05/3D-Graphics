import * as wasm from "../../../../wasm/wasm_graphics"

import React from 'react';
import { Accordion } from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { Trash2, CheckCircle2 } from 'lucide-react';
import TransformControls from './TransformControls';
import MaterialEditorControls from './MaterialEditorControls';

const wasmDeleteSelectedObject = () => {
    console.log("JS: wasmDeleteSelectedObject() - Calling wasm backend to delete selected object");
    wasm.delete_selected_object();
    wasm.set_follow_cursor(false);
};

const wasmConfirmObjectEdits = () => {
    console.log("WASM: Confirming edits for selected object");
    wasm.confirm_edits();
};

interface EditPanelProps {
    initialOpenSections?: string[];
    accordionKey?: string;
    onConfirmEdit: () => void;
    onDeleteObject: () => void;
    selectionVersion: number;
}

const EditPanel: React.FC<EditPanelProps> = ({ 
    initialOpenSections = ['transform', 'materialEditor'],
    accordionKey,
    onConfirmEdit,
    onDeleteObject,
    selectionVersion
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
            <div className="flex w-full gap-2">
                <Button
                    variant="default"
                    className="flex-1 min-w-0 bg-green-600 hover:bg-green-700 text-white"
                    onClick={handleConfirmClick}
                >
                    <CheckCircle2 className="mr-1 h-4 w-4" />
                    Confirm
                </Button>
                <Button
                    variant="destructive"
                    className="flex-1 min-w-0"
                    onClick={handleDeleteClick}
                >
                    <Trash2 className="mr-1 h-4 w-4" />
                    Delete
                </Button>
            </div>
            
            <Accordion 
                type="multiple" 
                defaultValue={initialOpenSections} 
                className="w-full"
                key={accordionKey} // Use the key here
            >
                <TransformControls />
                <MaterialEditorControls selectionVersion={selectionVersion} />
            </Accordion>
        </div>
    );
};

export default EditPanel;