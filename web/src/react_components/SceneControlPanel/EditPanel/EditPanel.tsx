import * as wasm from "@wasm/wasm_graphics"

import React from 'react';
import { Accordion } from "@/components/ui/accordion";
import { Button } from "@/components/ui/button";
import { Trash2, CheckCircle2 } from 'lucide-react';
import TransformControls from './TransformControls';
import MaterialEditorControls from './MaterialEditorControls';
import { useGameContext } from "@/gameContext";

interface EditPanelProps {
}

const EditPanel: React.FC<EditPanelProps> = ({}) => {

    const {
        selectedObjMatProps,
        gameStatus,
        followCamera
    } = useGameContext();

    const handleDeleteClick = () => {
        if (window.confirm("Are you sure you want to delete this object?")) {
            wasm.delete_selected_object();
        }
    };

    return (
        <div className="space-y-3">
            <div className="flex w-full gap-2">
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
                defaultValue={['transform-controls', 'material-editor']} // Example: open both by default
                className="w-full"
            >
                <TransformControls/> 
                <MaterialEditorControls/>
            </Accordion>
        </div>
    );
};

export default EditPanel;