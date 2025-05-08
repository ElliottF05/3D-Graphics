import * as wasm from '../../../wasm/wasm_graphics'

import React, { useEffect, useState } from 'react';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";

import EditPanel from './EditPanel/EditPanel';
import AddObjectPanel from './AddObjectPanel';

// --- Mock WASM Interaction ---

const wasmConfirmObjectEditsWasm = () => { // Renamed to avoid conflict with handler
    console.log("JS: Telling WASM edits are confirmed for selected object");
};
// --- End Mock WASM Interaction ---


const SceneControlPanel: React.FC = () => {

    // --- BE EXTREMELY CAREFUL WITH THIS, ENSURE WASM STATE IS IN SYNC ---
    // DESELECTION IN JS MUST BE COMMUNICATED TO WASM
    // USE wasm.wasm_deselect_object()
    const [isObjectSelected, setIsObjectSelected] = useState<boolean>(false);

    const [selectionVersion, setSelectionVersion] = useState<number>(0);
    const [activeMainAccordionItems, setActiveMainAccordionItems] = useState<string[]>(['add-object-panel']);
    const [editPanelOpenSubSections, setEditPanelOpenSubSections] = useState<string[]>(['transform']);
    const [editPanelAccordionKey, setEditPanelAccordionKey] = useState<string>('editPanelKey-initial');

    // Effect to register the React state setter to the global bridge
    useEffect(() => {
        (window as any).wasmBridge.jsSetIsObjectSelected = (isSelected: boolean) => {
            console.log(`JS: updateIsObjectSelectedFromWasm called by WASM with: ${isSelected}`);
            setIsObjectSelected(isSelected);

            if (isSelected) {
                // Query wasm backend for material properties
                setSelectionVersion(prevVersion => {
                    console.log("increasing selection version to force re-render:", prevVersion + 1);
                    return prevVersion + 1; // Increment version using the latest previous value
                });
            }
        };

        // Cleanup function when the component unmounts
        return () => {
            (window as any).wasmBridge.jsSetIsObjectSelected = (isSelected: boolean) => {
                console.warn("WASM tried to update selection, but React component (SceneControlPanel) was unmounted.");
            };
        };
    }, [setIsObjectSelected]); // Dependency: re-run if setIsObjectSelected changes (though unlikely for setters)


    const handleObjectAddedFromPanel = () => {
        setActiveMainAccordionItems(['edit-panel-wrapper']);
        setEditPanelOpenSubSections(['transform', 'materialEditor']);
    };

    const handleConfirmEditFromPanel = () => {
        wasmConfirmObjectEditsWasm();

        setIsObjectSelected(false);
        wasm.wasm_deselect_object();
        
        setActiveMainAccordionItems(prev => prev.filter(item => item !== 'edit-panel-wrapper')); // Close the EditPanel
        console.log("Edits confirmed, panel closed, object deselected.");
    };

    const handleDeleteObjectFromPanel = () => {
        // WASM's delete function should handle its internal selection state.
        // We just update React's view of selection.
        setIsObjectSelected(false);
        setActiveMainAccordionItems(prev => prev.filter(item => item !== 'edit-panel-wrapper')); // Close the EditPanel
        console.log("Object deleted, panel closed, object deselected in UI.");
    };

    const handleActiveMainAccordionChange = (newActiveItems: string[]) => {
        setActiveMainAccordionItems(newActiveItems);
    };

    const canOpenEditPanel = isObjectSelected;

    return (
        <Card className="w-full h-full overflow-y-auto rounded-none border-0">
            <CardHeader>
                <CardTitle>Scene Controls</CardTitle>
                <CardDescription>Manage and edit your 3D scene.</CardDescription>
            </CardHeader>
            <CardContent className="space-y-6 pb-20">
                <Accordion 
                    type="multiple" 
                    className="w-full"
                    value={activeMainAccordionItems}
                    onValueChange={handleActiveMainAccordionChange}
                >
                    {/* Add Object Panel Section */}
                    <AddObjectPanel onObjectAdded={handleObjectAddedFromPanel} />
                    
                    {/* Edit Panel Section Wrapper */}
                    <AccordionItem value="edit-panel-wrapper">
                        <AccordionTrigger
                                className={!canOpenEditPanel ? "text-muted-foreground/50 cursor-not-allowed" : ""}
                        >
                            Edit Selected Object
                        </AccordionTrigger>
                        <AccordionContent>
                            {canOpenEditPanel ? (
                                <EditPanel 
                                    initialOpenSections={editPanelOpenSubSections}
                                    accordionKey={editPanelAccordionKey}
                                    onConfirmEdit={handleConfirmEditFromPanel}
                                    onDeleteObject={handleDeleteObjectFromPanel}
                                    selectionVersion={selectionVersion}
                                />
                            ) : (
                                <p className="text-sm text-muted-foreground p-4 text-center">
                                    No object selected. Add an object or select one in the scene to edit.
                                </p>
                            )}
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </CardContent>
        </Card>
    );
};

export default SceneControlPanel;